# Contratos entre equipos

> Este es el documento que hace posible que tres personas trabajen en paralelo.
> Si algo no está aquí, no forma parte de la API pública del proyecto y puede
> cambiar sin previo aviso.

---

## 1. Principio

**Nadie implementa nada hasta que el contrato que consume está escrito.**

En el Sprint 1, antes de la primera línea de código, las tres personas firman
este documento. A partir de ese momento:

- Cada equipo codifica contra firmas fijadas, **aunque el otro aún no haya
  escrito la implementación**.
- Si una implementación resulta imposible o estúpida contra el contrato, eso se
  resuelve en el documento **primero**, no en el código después.
- Un cambio de contrato es un acto explícito: ADR + actualización de este
  documento + aviso el mismo día.

### Reglas de dependencia (no negociables)

```text
netscope-desktop  ──►  netscope-capture  ──►  netscope-protocol
        │                  │                     │
     C: interfaz        A: captura         B: protocolos

  · El primero depende de los otros dos.
  · El segundo depende del tercero.
  · El tercero no depende de nadie dentro del proyecto.
```

1. `netscope-protocol` no depende de ningún crate del proyecto.
2. `netscope-capture` no depende de `netscope-desktop`.
3. `netscope-desktop` no abre ficheros ni habla con el sistema operativo.
4. Nadie usa `pub(crate)` por el otro: lo que otro necesita es `pub`.
5. Nada de tipos duplicados entre crates. Si dos equipos necesitan lo mismo, el
   tipo vive en `protocol`.

---

## 2. Qué consume cada quien de quién

| Consumidor | Consume de | Nada más |
|-----------|-----------|----------|
| `capture` (A) | Tipos de `protocol`: `Packet`, `Layer`, `Protocol`, `Registry` | No lee ficheros de `desktop`, no sabe qué es un widget |
| `desktop` (C) | Tipos de `protocol` (modelo) + API de `capture` (almacén, filtros, estadísticas) | No lee ficheros por su cuenta, no llama a la biblioteca de captura |
| `protocol` (B) | Nada del proyecto | Solo librerías estándar |

### Superficie pública permitida por crate

`netscope-protocol` **expone** (los demás pueden usarlo):

- `Packet`, `Layer`, `Field`, `FieldValue`, `ByteRange`, `Protocol`,
  `LinkType`, `Timestamp`, `PacketIndex`
- `Registry` y `Decoder` (para añadir protocolos o registros propios)
- `DecodeError`, `DecodeErrorKind`
- `hexdump` y utilidades de presentación de bytes
- `TcpStream` / reensamblado (a partir del Sprint 7)

`netscope-protocol` **no expone**: internos del registro, representacion
intermedia mutable, ni nada que la interfaz tenga que interpretar por su cuenta.

`netscope-capture` **expone**:

- `PacketSource` y su enumerado de errores
- `PcapReader`, `PcapNgReader`, `LiveCapture`
- `PacketStore` (con índice, filtro y capacidad limitada)
- `Filter` y su sintaxis de texto
- `Stats` y sus instantáneas
- `Export` y `Generator`

`netscope-desktop` **expone**: nada. Es una hoja de arriba. (En el Sprint 3 se
extraerá un `netscope-cli` que consuma `capture`, por si se quiere el analizador
desde terminal; no cambia el DAG.)

---

## 3. Firmas de referencia

> **Borrador para el Sprint 1.** No son código: son las firmas que los tres
> equipos se comprometen a respetar. Se ajustan en la reunión de arranque y a
> partir de ahí solo cambian por ADR.

### De `protocol` a todos

```rust
/// Trama con marca de tiempo y metadatos de origen, todavía sin interpretar.
pub struct RawFrame { pub timestamp: u64, pub captured_len: u32,
                      pub original_len: u32, pub data: Vec<u8> }

/// Convierte bytes en el modelo. Función pura y sin estado.
pub struct Registry;
impl Registry {
    pub fn standard() -> Self;
    pub fn register(&mut self, d: impl Decoder) -> Result<(), RegistryError>;
    pub fn decode(&self, frame: &[u8], link: LinkType) -> Packet;
    pub fn decode_at(&self, frame: &RawFrame, link: LinkType,
                     index: PacketIndex, ts: Timestamp) -> Packet;
}

/// Contrato de un protocolo. Implementarlo es todo lo necesario para añadirlo.
pub trait Decoder: Send + Sync {
    fn protocol(&self) -> Protocol;
    fn selector(&self) -> Selector;              // EtherType, nº IP, puerto, enlace
    fn decode(&self, ctx: &mut LayerContext<'_>) -> Result<NextLayer, DecodeError>;
}
```

**Promesas de `protocol` que `capture` y `desktop` pueden apoyar:**

- `Packet::data` contiene siempre todos los bytes capturados.
- Todo `Field::range` es relativo al inicio de `packet.data`.
- `Field::range.slice(&packet.data)` devuelve `None` si el rango no cabe. Nunca
  hay acceso fuera de rango.
- Un `Packet` con errores sigue siendo utilizable: tiene las capas que sí se
  pudieron leer.
- `decode` no reserva memoria no acotada ni entra en bucle.

### De `capture` a `desktop`

```rust
pub trait PacketSource {
    fn info(&self) -> SourceInfo;
    /// Devuelve hasta `max` tramas; `timeout` es un tope, no una espera ciego.
    fn next_batch(&mut self, max: usize, timeout: Duration)
        -> Result<Vec<RawFrame>, SourceError>;
    fn close(&mut self) -> Result<(), SourceError>;
}

pub struct Pipeline;                 // fuente → registro → almacén → estadísticas
impl Pipeline {
    pub fn with_registry(registry: Registry) -> Self;
    pub fn ingest(&mut self, frame: RawFrame) -> PacketIndex;
    pub fn store(&self) -> &PacketStore;
    pub fn stats(&self) -> &Stats;
    pub fn visible(&self, filter: &Filter) -> impl Iterator<Item = &Packet>;
}

pub struct Filter;                   // protocolo, dirección, puerto, texto
impl Filter { pub fn parse(s: &str) -> Result<Filter, FilterError>;
              pub fn matches(&self, p: &Packet) -> bool; }
```

**Promesas de `capture` que `desktop` puede apoyar:**

- Los índices de `Packet` son estables y monotónicos durante una sesión.
- `Pipeline::ingest` es seguro de llamar desde el hilo de captura.
- El almacén nunca crece sin límite; si se alcanza la capacidad, descarta los
  más antiguos y lo dice en `Stats`.
- `Filter::parse` acepta el mismo lenguaje de texto que se muestra en la caja
  de búsqueda. Un filtro mal escrito se muestra como error de usuario, no como
  pánico.

---

## 4. Cómo trabaja cada quien sin el otro

Esta es la parte importante. Tres mecanismos:

### 4.1 Dobles de prueba (*doubles*)

- **C (interfaz)** trabaja con capturas de prueba desde el Sprint 2. No necesita
  captura en vivo ni generador: un `.pcap` de 20 paquetes bastan.
- **A (captura)** trabaja decodificando tramas que construye a mano en las
  pruebas. No necesita que exista ni un solo decodificador: el registro vacío ya
  devuelve un `Packet` válido.
- **B (protocolos)** trabaja con vectores de bytes escritos en el propio test.
  No necesita ficheros, ni red, ni interfaz.

### 4.2 Fixtures compartidas (`fixtures/`)

Un único directorio, propiedad compartida:

```text
fixtures/
├── minimos/        # 1 paquete por protocolo, terbaru y malformado
├── reales/         # capturas pequeñas y anonimizadas de tráfico propio
├── dorados/        # trama -> JSON esperado (propiedad de B)
└── carga/           # ficheros grandes para medir rendimiento (propiedad de A)
```

Regla: **cualquiera puede añadir una fixture; nadie la borra.** Un `.pcap` del
tráfico propio de quien lo aporta, anonimizado y de menos de 1 MB. Los `.pcap`
están en `.gitignore` por defecto; se suben a mano cuando alguien confirma que
están limpios.

### 4.3 Pruebas de contrato

Cada equipo escribe tests que consumen la API del otro usando solo lo prometido
en este documento. Si un test de contrato falla, **el problema está en el
contrato**, no en el que "todavía no ha terminado". Por eso las pruebas de
contrato se escriben en el Sprint 1, antes que la implementación.

---

## 5. Proceso para cambiar un contrato

1. **Aviso el mismo día** en el canal compartido: qué se cambia y por qué.
2. **ADR** en `docs/adr/` con: contexto, opciones, decisión, consecuencias y
   plan de migración.
3. **PR que actualiza este documento** junto con el código que lo rompe. Un PR
   que cambia una firma sin actualizar este fichero **no se aprueba**.
4. **Periodo de gracia de un sprint** para que los tres adapten su código, salvo
   que el cambio rompa la seguridad (entonces es inmediato).
5. Cambios aditivos (un campo nuevo, un método nuevo) no requieren ADR: basta
   actualizar este documento en el mismo PR.

### Qué es un cambio incompatible

- Quitar o renombrar un elemento público.
- Cambiar el tipo o el significado de un parámetro o de retorno.
- Cambiar la semántica de algo ya publicado (por ejemplo, que `timestamp` pase
  de microsegundos a nanosegundos **sin** actualizar la documentación).
- Cambiar el orden de un enum.

### Qué no es un cambio incompatible

- Añadir un elemento público nuevo.
- Añadir un protocolo nuevo.
- Cambiar la implementación interna manteniendo la firma.
- Arreglar un error que incumpla lo prometido (corregir el fallo de una promesa
  es una obligación, no una ruptura).

---

## 6. Matriz de dependencias por sprint

Qué necesita cada quien, y de quién, en cada momento del proyecto.

| Sprint | A necesita de B | B necesita de A | C necesita de A | C necesita de B |
|--------|-----------------|------------------|----------------|-----------------|
| 1 | Firma de `Registry`/`Decoder` | — | Firma de `PacketStore`/`Filter` | Modelo `Packet` |
| 2 | — (prueba con tramas escritas a mano) | Fixtures mínimas | Decodificador de algo que A pueda generar | Modelo y `Field` |
| 3 | — | Fixtures PCAPNG | Fichero grande para medir | Campos con offset |
| 4 | — | Fixtures de truncados | — | — |
| 5 | — | Fixtures de DNS/DHCP | — | Protocolos nuevos |
| 6 | — | Rendimiento: perfil de decodificación | Interfaz de captura | — |
| 7 | Generador para pruebas de flujo | Pruebas del generador | — | Reensamblado |
| 8 | Release | Release | Release | Release |

**Lectura de la tabla:** solo hay tres filas con dependencia real de trabajo, y
todas se resuelven con algo que existe desde el Sprint 1. En ningún momento
una persona tiene que esperar a otra para seguir avanzando.

---

## 7. Anti-patrones que romperían la independencia

Para reconocerlos a tiempo:

| Anti-patrón | Qué pasa | Cómo evitarlo |
|-------------|----------|---------------|
| "Espera, que me falta tu función" | Bloqueo entre sprints | Falta una promesa en el contrato |
| Un tipo que "vale para los dos" y vive en `capture` | La interfaz depende de la E/S | El tipo va en `protocol` |
| Fijar la posición de un campo por índice (`fields[3]`) | Cambia al añadir un campo | Buscar por nombre (`layer.field("TTL")`) |
| Un `match` sobre el protocolo dentro de la UI | La lógica se duplica y se pudre | Filtrar por `Protocol` desde el modelo |
| Un PR de 800 líneas de otro crate | Code review imposible | Un PR, un crate, una persona |
| El tipo del número de protocolo como `u8` suelto | Se comparan números por todas partes | Usar siempre `Protocol` |

---

## 8. Glosario rápido

| Término | Significado aquí |
|---------|-------------------|
| **Contrato** | Las firmas públicas y sus promesas, escritas en este documento |
| **Promesa** | Comportamiento que el documento garantiza y que se puede probar |
| **Crate** | Unidad de compilación de Cargo, equivalente a un módulo raíz |
| **DAG** | Grafo dirigido acíclico: la forma de las dependencias |
| **Fixture** | Fichero de captura de prueba, normalmente pequeño y versionado |
| **Golden** | Resultado esperado de una prueba, guardado para detectar cambios |
| **ADR** | Registro de decisión arquitectónica: contexto, opciones, decisión |
| **Hito de integración** | Momento del sprint en que los tres fusionan y comprueban juntos |