# Sprint 1 en detalle

> 10 días laborables. Objetivo único: **que nadie esté bloqueado durante los 15
> meses siguientes**. Nada de esto es funcionalidad del producto.

---

## Antes del lunes

- [ ] Las tres personas han leído `README.md`, `docs/ARQUITECTURA.md` y
      `docs/CONTRATOS.md`.
- [ ] La Persona B ha leído los ADR del 0001 al 0006.
- [ ] El workspace compila en el portátil de cada uno:
      `cargo test --workspace --no-default-features`
- [ ] Cada una sabe cuál es su carpeta y cuál es la de los demás.

---

## Reunión de arranque (lunes, 60 minutos)

El único resultado que importa: **`docs/CONTRATOS.md` firmado por los tres**.

| Minutos | Punto | Salida |
|---------|-------|--------|
| 0-10 | Por qué este reparto y no otro | Entendido |
| 10-25 | Los contratos, sección por sección | Objeciones por escrito |
| 25-40 | Qué promete cada quien y qué se compromete a no cambiar | Lista de promesas |
| 40-50 | ¿Falta alguna promesa que un equipo necesite del otro? | Frases añadidas |
| 50-60 | Reparto del sprint y ritmo de trabajo | 3 tareas por persona |

**Regla de la reunión:** no se debate implementación. Si algo es una decisión de
arquitectura, sale del sprint y se escribe como ADR.

---

## Plan día a día

### Persona A — Captura e ingesta

| Día | Tarea | Entregable comprobable |
|-----|-------|------------------------|
| 1 | Firmar la parte de `capture` del contrato | Sección editada y aprobada |
| 2 | Escribir `fixtures/minimos/`: 6 tramas escritas a mano con comentario | `.hex` de Ethernet/IPv4/UDP y variantes truncadas |
| 3 | `source.rs`: el trait `PacketSource` y su enumerado de errores | Compila, con un tipo de prueba en memoria |
| 4 | `store.rs`: `PacketStore` con índice y capacidad limitada | 20 pruebas unitarias de almacén |
| 5 | `filter.rs`: sintaxis de texto (`tcp`, `ip 10.0.0.1`, `port 443`, `a && b`, `!a`) | Parser y `matches()` probados |
| 6-7 | `pcap/file.rs`: cabecera global y registros, con sus cuatro números mágicos | Lee un `.pcap` de prueba del Sprint 1 |
| 8 | `stats.rs`: totales, por protocolo, caudal | Instantánea con valores esperados |
| 9 | Integración: `Pipeline` atando fuente → registro → almacén | Flujo completo con el registro vacío |
| 10 | Demo del sprint | Abre su propia captura y lista 10.000 paquetes |

### Persona B — Protocolos

| Día | Tarea | Entregable comprobable |
|-----|-------|------------------------|
| 1 | Firmar la parte de `protocol` del contrato | Sección editada y aprobada |
| 2 | `model.rs`: `Packet`, `Layer`, `Field`, `ByteRange`, `Protocol`, `LinkType` | Compila + pruebas de rango y de truncamiento |
| 3 | `error.rs`: `DecodeError` y sus causas | Compila |
| 4 | `decoder.rs`: `Reader`, `LayerContext`, el trait `Decoder` | Un decodificador de ejemplo en las pruebas |
| 5 | `registry.rs`: qué protocolo corresponde a qué identificador | El registro vacío y `standard()`, ambos probados |
| 6-7 | `layers/ethernet.rs` + `layers/ipv4.rs` completos | Trama buena, truncada y malformada por protocolo |
| 8 | `layers/udp.rs` con checksum y pseudo-cabecera | El mismo datagrama con el checksum bien y mal |
| 9 | Integración: el registro decodifica Ethernet + IPv4 + UDP encadenado | Un `.pcap` de `fixtures/` da tres capas |
| 10 | Demo del sprint | Explica un paquete con sus offsets, en voz alta |

### Persona C — Interfaz

| Día | Tarea | Entregable comprobable |
|-----|-------|------------------------|
| 1 | Firmar la parte de `desktop` del contrato | Sección editada y aprobada |
| 2 | `state.rs`: el estado de la aplicación, **sin egui** | Pruebas del estado sin abrir ventana |
| 3 | Maqueta de la ventana: tres paneles con datos falsos | Se ve la maqueta en pantalla |
| 4 | Lista de paquetes: columnas, selección, desplazamiento | Navega 1.000 filas inventadas |
| 5 | Panel de detalle: árbol de capas y campos con offset | Muestra campos inventados del modelo |
| 6 | Vista de bytes: volcado hexadecimal y ASCII | Un buffer inventado, resaltando una posición |
| 7 | Colores por protocolo y estados de error | Truncado y malformado se distinguen de un vistazo |
| 8 | Filtros: caja de texto y mensajes de error del DSL | Escribe `tcp && port 443` y filtra la maqueta |
| 9 | Integración: leer de `capture` con una fuente de prueba | Carga un `.pcap` de `fixtures/` real |
| 10 | Demo del sprint | Abre una captura y navega por un paquete |

---

## El momento clave: el lunes por la tarde

Si el día 1 no hay contrato firmado, **el sprint se para**. Nadie empieza a
implementar hasta que las tres firmas están. Es la única dependencia dura del
proyecto y cuesta medio día.

Cuando el contrato esté firmado, los tres pueden trabajar sin volver a
sincronizarse hasta el viernes.

---

## Definición de terminado (aplica a las tres)

- [ ] `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` limpio
- [ ] `cargo test --workspace --no-default-features` en verde
- [ ] cada cambio viene con pruebas: el caso bueno y **uno hostil**
- [ ] no rompe ninguna promesa de `docs/CONTRATOS.md`
- [ ] se demuestra en menos de dos minutos

---

## Hito del viernes (45 minutos)

1. Cada quien demuestra su entregable con la herramienta real.
2. Se ejecutan las pruebas de contrato cruzadas:
   - A prueba su código con el modelo y el registro de B.
   - C prueba su código con el almacén y los filtros de A.
   - B prueba sus decodificadores con las fixtures de A.
3. Se mira la lista de promesas y se comprueba que nada se rompió en silencio.
4. Se elige el backlog del Sprint 2.

**Si la demo de alguien depende de que otro haya terminado**, la integración
falla y el problema es del contrato, no de la persona. Se arregla añadiendo una
promesa, no esperando más.

---

## Qué NO se hace en el Sprint 1

| Tentación | Por qué no |
|-----------|-----------|
| Implementar la ventana de verdad | C hace una maqueta con datos falsos. Enseñar cosas falsas es más rápido que integrar todavía |
| Leer un `.pcap` real de producción | No hace falta: con 20 tramas escritas a mano se demuestra todo |
| Decidir la biblioteca de captura en vivo | Se decide en el Sprint 6, con un spike (ADR-0005) |
| Añadir un cuarto protocolo | Se hace en el Sprint 2. Aquí solo el esqueleto |
| Escribir la interfaz bonita | Ahora importa la estructura, no el color |

---

## Riesgos del sprint y qué hacer si pasan

| Si pasa esto | Entonces |
|--------------|----------|
| El modelo de B no sirve para lo que C quiere mostrar | **Hoy**, no el viernes. C enseña su maqueta el día 3 y se ajusta el modelo. Por eso la maqueta va tan pronto |
| La firma de lectura de A no encaja con lo que necesita C | Día 2: se escribe el doble de prueba de C sobre la firma actual. Si no encaja, se cambia la firma el día 3 |
| Alguien no ha terminado el día 9 | Se recorta alcance, nunca calidad: una tarea a medias con pruebas vale más que dos a medias sin ellas |
| Alguien sobra en su sprint | Nadie sobra: B puede adelantar el ARP, C puede adelantar las estadísticas, A puede adelantar el generador |