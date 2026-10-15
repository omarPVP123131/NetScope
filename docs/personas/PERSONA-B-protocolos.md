# Persona B — Protocolos

> Crate: **`netscope-protocol`**. Frase: *¿qué significa cada byte?*

---

## Tu misión en una frase

Convertir bytes arbitrarios en campos con nombre, valor y posición, **sin
caerse nunca**, por muy roto que venga el fichero.

## Lo tuyo

| Módulo | Qué hace | Sprint |
|--------|----------|--------|
| `model.rs` | `Packet`, `Layer`, `Field`, `ByteRange`, `Protocol`, `LinkType` | 1 |
| `decoder.rs` | `Reader`, `LayerContext`, el trait `Decoder` | 1 |
| `registry.rs` | Qué protocolo corresponde a qué identificador | 1 |
| `error.rs` | `DecodeError` y sus causas | 1 |
| `hexdump.rs` | Volcado hexadecimal y ASCII | 2 |
| `layers/ethernet.rs` | Ethernet II | 2 |
| `layers/ipv4.rs` | IPv4 | 2 |
| `layers/udp.rs` | UDP | 2 |
| `layers/arp.rs` | ARP | 3 |
| `layers/tcp.rs` | TCP, con checksum y pseudo-cabecera | 3 |
| `layers/icmp.rs` | ICMPv4 e ICMPv6 | 3 |
| `layers/dns.rs` | DNS | 5 |
| `layers/dhcp.rs` | DHCP | 5 |
| `layers/ipv6.rs` | IPv6, incluidas las cabeceras de extensión | 4 |
| `flow.rs` | Reensamblado de flujos TCP | 7 |

## Lo que no es tuyo

- De dónde vienen los bytes (lo sabe A) y dónde se muestran (lo sabe C).
- **El modelo se congela en el Sprint 1.** Después, cambiarlo es un ADR. Si
  necesitas un campo nuevo para un protocolo nuevo, se añade. Cambiar la
  *estructura* es una decisión de arquitectura.

---

## Decisiones de diseño ya tomadas

No hace falta volver a pensarlas. Están en
[`../ARQUITECTURA.md`](../ARQUITECTURA.md) y en el ADR-0002.

1. **Los campos no copian los bytes.** Un `Field` guarda un `ByteRange` (offset y
   longitud) dentro del buffer del paquete. Un paquete de 1.500 bytes con diez
   capas cuesta **una** asignación, no once.
2. **El paquete es dueño de su buffer.** Los `Range` solo son válidos mientras
   ese paquete exista. Quien lo guarde, guarda también los bytes.
3. **Toda lectura pasa por un lector con límites comprobados.** No hay indexado
   directo en ningún sitio. Si el rango no cabe, se devuelve error o `None`.
4. **Un paquete con errores sigue siendo un paquete.** Se marca y se muestra.
   Un fichero corrupto no puede hacer que el programa deje de funcionar.
5. **Un protocolo se añade, no se modifica.** Un archivo nuevo en `layers/` y
   una línea en el registro. Ni `match` gigantes, ni Flags condicionales
   repartidos.

---

## Anatomía de un decodificador

```text
1. El registro elige un Decoder según lo que haya en la trama
   (EtherType, número de protocolo IP o puerto).

2. Se crea un LayerContext con los bytes de la capa.

3. El Decoder:
   - lee con el Reader (devuelve error si no cabe)
   - escribe campos con ctx.push(...)
   - avisa de problemas no fatales con ctx.warn(...)
   - devuelve el NextLayer: qué va debajo

4. Si hay error fatal, el registro guarda la capa parcial y el error,
   y para. Nunca descarta el paquete entero.
```

### Por qué `NextLayer` y no un `match` central

Porque el orden de las capas **depende del contenido de la trama**: si el
EtherType es 0x0800 se abre IPv4, y si el campo *protocolo* de esa cabecera es
17 se abre UDP. Un `match` central obligaría a que la capa de enlace supiera de
IP. Con `NextLayer`, cada protocolo decide qué viene después y el registro solo
resuelve.

Es el mismo motivo por el que DNS se detecta por **puerto** y no por EtherType:
es el primer caso donde la capa depende del transporte.

---

## Modelo de datos

| Tipo | Qué es | Trampa |
|------|--------|--------|
| `Packet` | Índice, marca de tiempo, longitudes, bytes, capas, errores | `captured_len` ≠ `original_len` cuando el `snaplen` cortó el paquete |
| `Layer` | Un protocolo, su rango y sus campos | El rango se recorta a la longitud **declarada**, no a la disponible |
| `Field` | Nombre, valor, rango, nota | Buscar por nombre, nunca por posición |
| `ByteRange` | Offset y longitud dentro de la trama | Su `slice` devuelve `None` si no cabe. Nunca indexar |
| `Protocol` | Identidad de un protocolo | Enum cerrado. Fuera de aquí, nadie compara números de EtherType |
| `DecodeError` | Problema encontrado, con dónde | No invalida el paquete |

**Los tres niveles de error**, que hay que distinguir bien:

| Nivel | Ejemplo | Qué hace el registro |
|-------|---------|----------------------|
| Fatal para esa capa | Longitud declarada menor que la cabecera | Guarda lo que haya y para |
| No fatal | Checksum que no cuadra, protocolo desconocido | Anota y sigue |
| Sin efecto | Longitud declarada mayor que los datos | Trunca y avisa: el paquete llegó incompleto |

Confundir el segundo con el primero es el error de diseño más caro de este
crate: si el checksum lo tratamos como error fatal, medio programa
deja de funcionar.

---

## Orden de ataque recomendado

1. **Ethernet**: 14 bytes fijos, el ejemplo más legible.
2. **IPv4**: el decodificador de referencia. Longitud variable, fragmentación,
   checksum, longitudes imposibles. Si algo de aquí no está bien, nada más lo
   estará.
3. **UDP**: 8 bytes fijos, checksum opcional en IPv4 y obligatorio en IPv6. El
   ejemplo más pequeño de pseudo-cabecera.
4. **TCP**: más campos, mismo patrón. El checksum obliga a que la capa IP
   comunique las direcciones: es el primer momento en que hace falta estado
   entre capas.
5. **ARP, ICMP, DNS, DHCP**: ya con el patrón aprendido, cada uno es corto.
6. **IPv6**: cabecera fija, pero con la cadena de cabeceras de extensión.
7. **Reensamblado**: requiere los cuatro anteriores bien.

---

## Pruebas: el mínimo de cada decodificador

Un decodificador sin estas pruebas **no está terminado**:

- [ ] Trama completa y correcta.
- [ ] Truncada en cada longitud crítica (cabecera justa, cabecera - 1, a la
      mitad del payload).
- [ ] Con el campo de longitud declarado imposible (menor que la cabecera,
      enorme).
- [ ] Con el checksum bien y con el checksum roto.
- [ ] Con longitud variable en el mínimo y en el máximo.
- [ ] Con un fragmento, si el protocolo tiene fragmentación.
- [ ] Un test del **corpus hostil** (tramas mutadas a mano): **no puede haber pánico**.

El mensaje de error es parte del producto: en la prueba, comprueba que dice
algo útil, no solo que tiene el tipo correcto.

---

## Corpus hostil (desde el Sprint 2, no al final)

No necesita interfaz, ni ficheros, ni red: es un conjunto de tramas escritas a
mano —mutadas de forma deliberada— que se pasan por el decodificador para
comprobar que no hay pánico y que termina.

```text
trama aleatoria → decode → ¿se ha colgado? ¿ha petado?
```

Corre en segundos y encuentra más fallos que cualquier revisión manual. No
obstante, **el corpus no sustituye a entender el protocolo**: encuentra que el
parser se rompe con una trama concreta, no por qué la trama es inválida.

> Si en algún momento sobra tiempo (o el proyecto crece), `cargo-fuzz` genera el
> corpus solo y es lo primero que se añade. Ver la nota de
> [`../ARQUITECTURA.md`](../ARQUITECTURA.md#8-estrategia-de-pruebas).

---

## Rendimiento

| Presupuesto | Cómo se llega |
|-------------|--------------|
| 1.000.000 de paquetes en menos de 2 s | Un campo es un rango, no una copia; ninguna asignación por cabecera |
| El registro no busca en un vector | Tabla de dispersión por EtherType, número de protocolo y puerto |
| Un paquete con bucle de decodificación termina | Tope de capas por paquete, y conjunto de visitados |

La segunda es la que más sorprende: un `Vec` de decodificadores buscado por
filtro lineal es rápido en un decodificador y lentísimo en cien mil paquetes.

---

## Criterios de aceptación

- [ ] Cero `unsafe` (prohibido por el workspace).
- [ ] Ningún indexado directo: todo pasa por el lector con límites.
- [ ] Ningún `unwrap`, `expect` ni `panic!` en camino de ejecución normal con
      datos externos.
- [ ] Ningún campo de longitud se usa para reservar memoria sin tope.
- [ ] Cada protocolo tiene sus pruebas completas, truncadas y malformadas.
- [ ] El corpus hostil no encuentra pánicos.
- [ ] Añadir un protocolo nuevo es un archivo y una línea.

---

## Recursos

- Tu guía de estudio por protocolo, con las trampas de cada uno:
  [`../PROTOCOLOS.md`](../PROTOCOLOS.md)
- Errores de seguridad que hay que evitar:
  [`../ARQUITECTURA.md`](../ARQUITECTURA.md#9-seguridad-del-parser)
- Tus contratos exactos: [`../CONTRATOS.md`](../CONTRATOS.md)
- Especificaciones: RFC 791 (IPv4), 8200 (IPv6), 768 (UDP), 9293 (TCP),
  826 (ARP), 1035 (DNS).

## Lo primero que haces mañana

1. Leer `docs/CONTRATOS.md` y proponer cambios si el modelo no sirve para lo que
   C necesita mostrar.
2. Escribir `model.rs` con sus pruebas, en especial las de rango y truncamiento.
   **Es tu primer entregable del Sprint 1**, porque de él dependen las otras dos
   personas.