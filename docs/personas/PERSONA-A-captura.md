# Persona A — Captura e ingesta

> Crate: **`netscope-capture`**. Frase: *¿de dónde salen los bytes y qué se hace
> con ellos?*

---

## Tu misión en una frase

Que el programa tenga paquetes, reliably y sin romperse, venga de donde venga.

## Lo tuyo

| Módulo | Qué hace | Sprint |
|--------|----------|--------|
| `source.rs` | El trait que unifica fichero y captura en vivo | 1 |
| `store.rs` | El almacén: índice, capacidad, consulta | 1 |
| `filter.rs` | Filtros y su sintaxis de texto | 1 |
| `stats.rs` | Totales, distribución por protocolo y caudal | 1 |
| `pipeline.rs` | Atar fuente → registro → almacén → estadísticas | 1 |
| `pcap/file.rs` | Lector de PCAP clásico | 2 |
| `pcap/pcapng.rs` | Lector de PCAPNG | 3 |
| `capture/` | Captura en vivo, por sistema operativo | 6 |
| `export.rs` | Exportar una selección a `.pcap` | 5 |
| `generator.rs` | Generador de paquetes de laboratorio | 7 |

## Lo que no es tuyo

- Interpretar un campo de protocolo. **No** sabes qué es un número de secuencia
  TCP: lo pide a `netscope-protocol` y guardas lo que te devuelvan.
- La ventana. C no sabe qué fichero se está leyendo, y tú no sabes cuántas
  filas se pintan.

---

## Detalle técnico por módulo

### `source.rs` — la abstracción

Todo lo que dé paquetes implementa el mismo trait. Es lo que permite que la
captura en vivo llegue en el Sprint 6 sin tocar ni una línea de la interfaz ni del
pipeline.

```rust
pub trait PacketSource {
    fn info(&self) -> &SourceInfo;
    fn next_batch(&mut self, max: usize, timeout: Duration)
        -> Result<Vec<RawFrame>, SourceError>;
    fn close(&mut self) -> Result<(), SourceError>;
}
```

Tres decisiones ya tomadas, para que no vuelvas a pensarlas:

1. **Se entregan lotes, no paquetes de uno en uno.** `next_batch` con un tope de
   tiempo. Leer 1.000 de golpe cuesta una fracción de lo que cuesta ir de uno en
   uno, y el timeout es un tope, no una espera ciega.
2. **Una trama cruda es solo bytes + marca de tiempo + longitudes.** Nada de
   interpretación. Cualquier cosa que huela a protocolo es cosa de B.
3. **Los errores son de la fuente**, no de `std::io`: un fichero con la cabecera
   rota no es «fin de fichero», es un error de formato con mensaje.

### `pcap/file.rs` — el formato PCAP

Es un formato sencillo, con dos trampas que hacen perder horas a quien no las
conoce.

**Cabecera global (24 bytes).** El número mágico decide dos cosas a la vez:

| Número mágico | Lectura | Marca de tiempo |
|---------------|---------|-----------------|
| `0xa1b2c3d4` | big-endian | microsegundos |
| `0xd4c3b2a1` | little-endian | microsegundos |
| `0xa1b23c4d` | big-endian | **nanosegundos** |
| `0x4d3cb2a1` | little-endian | **nanosegundos** |

Si el número no es ninguno de los cuatro, el fichero no es un PCAP. Punto. El
resto de la cabecera: versión mayor y menor, `thiszone` (ignóralo), `sigfigs`
(ignóralo), `snaplen` y `network`, que es el tipo de enlace.

**Registro de paquete (16 bytes).** `ts_sec`, `ts_frac`, `incl_len` (los bytes
guardados) y `orig_len` (los bytes que tenía el paquete en el medio).

```text
solo se leen incl_len bytes, nunca orig_len
incl_len > orig_len  → fichero corrupto
orig_len > snaplen  → normal: el paquete venía más grande que el snaplen
```

**Las tres trampas:**

1. **Leer `orig_len` bytes cuando solo hay `incl_len` guardados.** Es el error
   clásico, y produce basura o un pánico. `incl_len` es el único número que
   manda sobre cuánto se lee.
2. **Suponer que `snaplen` es 1500 o 65535.** Hay quien pone 96 bytes. Es una
   decisión de quien capturó, no del formato.
3. **Ignorar el endianness.** Los cuatro números mágicos existen justamente
   porque un fichero puede estar escrito al revés.

**Marca de tiempo.** El modelo usa microsegundos desde la época Unix. Si el
fichero es de nanosegundos, divide entre 1000 al leer, una vez, y nunca más.

**Tope de seguridad.** `incl_len` puede decir 4.000.000. Si exceeds un límite
razonable, hay dos salidas válidas: leerlo en un buffer acotado y truncar el
paquete, o cortar la lectura. Lo primero es más fiel. Lo segundo es más
defensivo. **Decídelo el Sprint 2 y escríbelo en el código, no en tu cabeza.**

### `pcap/pcapng.rs` — el formato PCAPNG (Sprint 3)

Bloques de longitud y tipo, con un número mágico de endianness propio
(`0x1A2B3C4D`) dentro del propio bloque. Tres bloques que te importan:

- **SHB**: cabecera de sección. Empieza una sección; puede haber varias en el
  mismo fichero.
- **IDB**: una interfaz y sus parámetros (`snaplen`, `linktype`). Un fichero
  puede tener paquetes de varias interfaces mezclados: **el tipo de enlace
  cambia de un bloque al siguiente**.
- **EPB**: un paquete, con su marca de tiempo en nanosegundos, longitudes y datos,
  y opcionalmente el identificador de interfaz al que pertenece.

Todo lo demás (`NRB`, `ISB`, opciones) se puede ignorar legítimamente al leer.

### `store.rs` — el almacén

```text
índice del paquete  →  Packet
orden de llegada    →  [índice, índice, índice, ...]
```

Dos estructuras: un mapa para buscar por índice en O(1) y una cola ordenada para
recorrer en orden de llegada. Cuando se alcanza la capacidad, se descarta el
más antiguo y se cuenta. La capacidad se ve en las estadísticas.

**El índice es estable y monotónico durante toda la sesión**, aunque el paquete
ya no esté en el almacén. Es lo que permite que la interfaz diga «el paquete
nº 47.000 ya no está» sin que sea un error.

### `filter.rs` — la sintaxis

El mismo lenguaje que se escribe en la caja de búsqueda de la interfaz y el que
acepta una función de filtrado. Ejemplos que tienen que funcionar:

```text
tcp                      solo TCP
ip 10.0.0.1              cualquier paquete con esa dirección
10.0.0.1:443             dirección y puerto
src 10.0.0.1:80          por dirección de origen
port 443                 cualquier paquete con ese puerto
tcp && port 80           conjunción
tcp || udp               disyunción
!arp                     negación
arp && 10.0.0.1          combinación de los dos
```

**Prioridad de operadores:** `!` > `&&` > `||`. Un filtro mal escrito es un
**error de usuario con mensaje**, no un pánico ni un filtro silencioso que no
muestra nada. Es la diferencia entre «no hay resultados» y «has escrito mal la
búsqueda».

### `pipeline.rs` — el punto de unión

```text
RawFrame → Registry::decode → Packet → PacketStore + Stats
```

Es el único sitio donde se junta todo. Se puede llamar desde el hilo de captura
sin bloquear la interfaz.

### `capture/` — tiempo real (Sprint 6)

El único código del proyecto específico del sistema. Va detrás del trait, con
un módulo por sistema y un módulo `unsupported` que devuelve un error claro en
lugar de fallar al arrancar.

Antes de escribirlo: **spike de dos días** comparando al menos dos bibliotecas
(ADR-0005). Y un detalle que se olvida: capturar requiere permisos de
administrador en la mayoría de sistemas. Está bien, es una señal de alarma
correcta, pero hay que explicarlo en el mensaje de error.

---

## Rendimiento

Presupuestos que se miden y se publican (Sprint 4):

| Operación | Objetivo |
|-----------|----------|
| Abrir un `.pcap` de 100 MB | menos de 3 segundos |
| Filtrar 100.000 paquetes | menos de 20 ms |
| Guardar 1.000.000 de paquetes | memoria acotada y conocida |

El orden correcto de optimización es: **evitar trabajo** (no decodificar lo que
el filtro descarta), **no copiar**, **lotes en vez de unidad**, y solo entonces
el código.

---

## Criterios de aceptación de tus módulos

- [ ] Ningún fichero, por roto que esté, termina el programa.
- [ ] Un paquete truncado se guarda y se marca; no se descarta.
- [ ] Todos los errores de lectura tienen un mensaje que una persona entiende.
- [ ] Cada módulo tiene sus pruebas y al menos un caso con entrada hostil.
- [ ] El almacén no crece sin límite y lo dice.

---

## Recursos

- Especificación del formato: [PCAP File Format](https://www.tcpdump.org/pcap/pcap.html)
  y [PCAPNG](https://pcapng.github.io/pcapng/)
- Tu checklist de entrada hostil está en
  [`../ARQUITECTURA.md`](../ARQUITECTURA.md#9-seguridad-del-parser)
- Tus contratos exactos: [`../CONTRATOS.md`](../CONTRATOS.md)

## Lo primero que haces mañana

1. Leer `docs/CONTRATOS.md` y proponer cambios si algo no encaja.
2. Escribir seis tramas en `fixtures/minimos/`, cada una con un comentario que
   explique qué caso cubre. **Es tu primer entregable del Sprint 1**, porque de
   ellas dependen las otras dos personas.