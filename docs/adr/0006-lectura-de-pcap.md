# ADR-0006 · Lectura de PCAP y PCAPNG: código propio, biblioteca o ambas

**Estado:** **aceptada** · **Decidida:** 14 de octubre de 2026 ·
**Personas afectadas:** A y B

> Esta decisión estaba prevista para el primer día del Sprint 2. Se ha adelantado
> porque condiciona el `Cargo.toml`, y cambiarlo con los tres equipos ya
> escribiendo código es más caro que decidirlo antes.

---

## Contexto

Hay dos formatos que hay que leer:

- **PCAP clásico**: 24 bytes de cabecera global y 16 por registro. Sencillo.
- **PCAPNG**: bloques de longitud y tipo, con secciones, varias interfaces,
  endianness por sección y opciones. No es sencillo.

El criterio del proyecto es doble: aprender a leer formatos binarios **y**
mantener el producto instalable sin dependencias del sistema.

## Candidatas (datos de septiembre–octubre de 2026)

| Biblioteca | Descargas | Mantenimiento | Cubre | Notas |
|------------|-----------|---------------|-------|-------|
| [`pcap-file`](https://docs.rs/pcap-file/) | 13,9 M acumuladas, ~4 M al mes | **Actualizado hace días** | PCAP y PCAPNG, lectura **y escritura**, zero-copy | La más usada y la más viva del ecosistema |
| [`pcap-parser`](https://docs.rs/pcap-parser/latest/pcap_parser/) | 1,1 M acumuladas, ~148 k al mes | Actualizado hace ~1 año | PCAP y PCAPNG, cero copias, código seguro | Del proyecto Rusticata. Correcta, pero tranquila |
| `pcarp` | 209 mil acumuladas | Sin actualizar desde 2022 | Solo PCAPNG | Descartada por abandono |
| `pcap` | 7,5 M acumuladas | Activa | Solo captura en vivo | Depende de libpcap del sistema |
| `capfile` | Reciente | Nuevo | PCAP y PCAPNG + disección | Casi sin usar; sin historial quevalorar |

Los datos provienen del [índice de palabras clave de crates.io para
pcapng](https://crates.io/keywords/pcapng) y para
[pcap](https://crates.io/keywords/pcap).

---

## Decisión

### 1. PCAP clásico: **código propio**

`netscope-capture/src/pcap/file.rs` se escribe a mano en el Sprint 2.

- Son unas 300–400 líneas y es exactamente la parte formativa del proyecto.
- Quita la dependencia más pesada del crate para la mitad del producto.
- No obliga a mantener una versión de Rust concreta ni a mirar la API de nadie.

### 2. `pcap-file` como **dependencia de desarrollo**, no como dependencia

Se añade en `[dev-dependencies]` de `netscope-capture`, y **solo en pruebas**.

Sirve para dos cosas, que son las que de verdad importan:

- **Pruebas diferenciales.** El mismo fichero se lee con nuestro lector y con
  el suyo; los dos tienen que dar **el mismo número de paquetes, las mismas
  longitudes y las mismas marcas de tiempo**. Si no coinciden, o el nuestro
  está mal o el fichero es raro, y hay que averiguarlo.
- **Generación de ficheros de prueba.** Cualquier Rareza del formato
  (endianness al revés, nanosegundos, secciones múltiples) se fabrica con la
  biblioteca y se lee con el nuestro.

Así no se depende de ella en tiempo de ejecución, pero alguien con más
experiencia nos está revisando el parser desde las sombras.

### 3. PCAPNG: **`pcap-file` para leer**

Sprint 3. El formato tiene secciones, interfaces y endianness por sección;
implementarlo desde cero es gasto sin valor docente. Se usa la biblioteca
detrás del mismo `PacketSource`, así que la diferencia con un `.pcap` solo se ve
en el módulo de lectura.

**Nuestro escritor** (`generator.rs`, `export.rs`) escribe **PCAP clásico**,
que es de una sola vez y controlable. El generador de paquetes del Sprint 7
comprobado así: el fichero que escribimos, lo lee Wireshark y lo lee la
biblioteca.

### 4. Captura en vivo: sigue abierta

La decisión de la biblioteca de captura se toma en el Sprint 6 con el spike
definido en [ADR-0005](0005-captura-en-vivo.md). Este ADR no la prejuzga.

---

## Consecuencias

**A favor**

- Se aprende de verdad el formato PCAP, que es el objetivo del proyecto.
- El producto no depende de ninguna biblioteca para abrir ficheros.
- La validación no es autoinforme: alguien independiente comprueba el parser.
- El formato raro se cubre sin escribir a mano cada variante.

**En contra**

- Más código que revisar en el Sprint 2 (unas 300 líneas más de lo previsto).
- Dos implementaciones de lectura conviven: el mantenimiento de la nuestra es
  nuestro, y la de la biblioteca no es problema nuestro. Eso está documentado.
- Las pruebas diferenciales hay que escribirlas: si no, `pcap-file` se queda sin
  usarse y es una dependencia muerta.

## Cómo saber si esta decisión fue mala

Se revisa al final del Sprint 3, con estos tres números:

| Señal | Lectura |
|-------|---------|
| El lector propio costó **más de una semana** | El presupuesto estaba mal: el Sprint 3 se retrasa |
| Las pruebas diferenciales encontraron más de un fallo real | Ha merecido la pena: el parser es mejor de lo que creíamos |
| El Sprint 7 se atasca escribiendo ficheros que otros no leen | El escritor propio no era el problema; se delega en la biblioteca |

## Alternativas descartadas

- **Solo biblioteca**: oculta justo la parte del proyecto que más conviene
  entender, y ata el producto al ritmo de mantenimiento de un tercero.
- **Solo código propio, también para PCAPNG**: tres sprints de formato binario
  antes de ver un paquete. Es gastar el presupuesto del MVP en el contenedor.
- **`pcap-parser` en lugar de `pcap-file`**: cubre lo mismo, con diez veces
  menos uso y sin actividad reciente.
- **`pcap` para todo**: arrastra libpcap o Npcap a la instalación completa.