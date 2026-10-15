# Persona C — Interfaz y análisis

> Crate: **`netscope-desktop`**. Binario: **`netscope`**. Frase: *¿cómo se ve lo
> que ya está interpretado?*

---

## Tu misión en una frase

Que alguien que no ha escrito el código pueda mirar un paquete y entenderlo.

## Lo tuyo

| Módulo | Qué hace | Sprint |
|--------|----------|--------|
| `state.rs` | El estado de la aplicación, **sin egui** | 1 |
| `main.rs` / `app.rs` | Arranque y composición de la ventana | 1-4 |
| `views/lista.rs` | Tabla de paquetes, columnas, selección | 2 |
| `views/detalle.rs` | Árbol de capas y campos con offset | 3 |
| `views/bytes.rs` | Volcado hexadecimal y ASCII | 4 |
| `views/filtros.rs` | Caja de filtros, búsqueda y mensajes | 5 |
| `views/estadisticas.rs` | Totales y distribución | 4-6 |
| `views/flujo.rs` | Reensamblado de flujos TCP | 7 |
| `views/export.rs` | Exportar la selección | 5-7 |

## Lo que no es tuyo

- Abrir ficheros. Se lo pides a `netscope-capture`.
- Interpretar bytes crudos. Todo lo que pintas **ya viene interpretado** en el
  modelo. Si necesitas un dato que no está, es que falta en el modelo: se pide a
  B, no se calcula aquí.

---

## La regla que define tu crate

```text
state.rs   ← lógica, datos, filtros, selección. CERO egui.
views/     ← egui. Solo pinta lo que hay en state.rs.
```

Por qué: `state.rs` se puede probar en un `cargo test` en medio segundo, sin
ventana, sin pantalla y sin bloquearse con un bucle de eventos. Y el día que
alguien quiera una versión de terminal, `state.rs` ya sirve tal cual.

Si te ves tentado a meter lógica en una vista, es que falta algo en `state.rs`.

---

## Anatomía de la ventana

```text
┌──────────────────────────────────────────────────────────────┐
│ barra superior: abrir · capturar/parar · filtros · exportar  │
├────────────────────────────┬─────────────────────────────────┤
│                            │  detalle                        │
│   lista de paquetes        │  capas y campos con offset       │
│                            ├─────────────────────────────────┤
│                            │  vista de bytes (hex + ASCII)    │
├────────────────────────────┴─────────────────────────────────┤
│ barra de estado: paquetes · bytes · errores · duración        │
└──────────────────────────────────────────────────────────────┘
```

Cuatro decisiones:

1. **La lista es el centro de gravedad.** Es lo que se mira el 90 % del tiempo.
2. **Detalle y bytes están apilados, no en pestañas.** Elegir un campo y ver su
   posición en la trama a la vez es el uso principal.
3. **La barra de estado no es decorativa.** Es donde vive «150.000 paquetes,
   42 MB, 317 con errores, captura de 0,8 s» y donde se ve si algo va mal.
4. **El filtro está arriba, no escondido.** Es la función que más se usa.

---

## Por módulo

### `state.rs` — primero, y sin egui

Qué guarda, como mínimo:

```text
paquete seleccionado          filtro activo
texto de búsqueda             desplazamiento de la lista
vista de bytes                estado de la captura (inactiva / capturando)
capacidad del almacén         mensajes para el usuario
```

Y qué ofrece:

- `abrir(fichero)`
- `seleccionar(índice)`
- `visibles()` → los paquetes que pasan el filtro
- `paquete_seleccionado()`
- `campo_seleccionado()` → para el resaltado en la vista de bytes

Todo testeable sin ventana. Si necesitas que egui te llame cada fotograma,
añade aquí un `tick()` y el estado se actualiza igual en las pruebas.

### `views/lista.rs` — la tabla

Columnas que la primera versión necesita:

```text
# │ Hora │ Origen │ Destino │ Protocolo │ Longitud │ Info
```

Detalles que importan más de lo que parecen:

- **Virtualización.** 150.000 filas no se pintan todas. egui tiene
  `ScrollArea::show_rows`, que solo dibuja las visibles. Sin esto, la lista se
  congela a los 5.000 paquetes y el proyecto se muere en el Sprint 4.
- **El filtro se aplica antes de pintar**, nunca durante.
- **Los errores se ven de un vistazo**: un icono o color distinto para truncado,
  malformado y desconocido. Es el requisito de que el programa «no se caiga»
  sea visible.
- **El color por protocolo** es una tabla, no un `match` repartido por el
  código. Añadir un protocolo es añadir una fila.

### `views/detalle.rs` — el árbol de capas

Por cada capa, los campos con: **nombre, valor y offset**. El offset es la
columna que hace útil el programa: conecta lo que se lee con lo que hay en la
trama.

- El árbol viene **del modelo**, no lo construyes comparando protocolos.
- Las notas de un campo (`with_note`) salen en un tooltip.
- Un campo truncado, sin bytes disponibles, se muestra con su valor pero vacío
  en la vista de bytes, no con un error.
- El clic en un campo selecciona su rango en la vista de bytes. **Es el
  conector entre los dos paneles y hay que implementarlo bien.**

### `views/bytes.rs` — el volcado

- Hexadecimal a la izquierda, ASCII a la derecha, con el offset al principio de
  la línea: el formato clásico, que es lo que todo el mundo reconoce.
- **Resaltado del rango seleccionado** desde el panel de detalle.
- Colores distintos para origen, destino y carga útil, solo si no molestan.
- Botón de copiar: hexadecimal plano, útil para pegar en una incidencia.
- Se dibuja solo la ventana visible. Un `.pcap` grande tiene millones de bytes.

### `views/filtros.rs` — la caja de búsqueda

- Escribes `tcp && port 443` y filtra al instante.
- **El mensaje de error del parser es tuyo de cara al usuario**: «No entiendo la
  parte `puerto`» es mejor que «error de sintaxis».
- Historial de filtros con las teclas arriba y abajo.
- Los filtros que usen guardados se pulsan, no se escriben.

### `views/estadisticas.rs`

Totales, distribución por protocolo, puertos más frecuentes y caudal. Vienen
calculados de `stats.rs`: tú los pintas. El dato que de verdad importa es
**cuántos paquetes hay con errores**, porque es la medida de si un fichero es
limpio.

---

## Rendimiento

La interfaz tiene que verse igual con 10 paquetes que con 150.000:

| Objetivo | Cómo se cumple |
|----------|----------------|
| 60 fps con 10.000 paquetes visibles | Virtualizar la lista; filtrar antes de pintar |
| Abrir un `.pcap` grande sin congelarse | Leer por lotes y mostrar progreso, no una espera |
| El hilo de captura nunca bloquea el pintado | El estado se actualiza en `tick()`, no en un bucle bloqueante |
| Abrir y cerrar 1.000 veces sin fugas | egui lo gestiona, pero no guardes clones de cada paquete |

**Si algo tarda más de 100 ms**, el patrón correcto es: trabajar por lotes y
volver a pintar, no congelarse hasta terminar.

---

## Errores de interfaz que importan

- Un filtro que no devuelve resultados **no es un error**: dilo («0 paquetes
  coinciden») para que no parezca que el programa se ha roto.
- Un fichero corrupto **no cierra la aplicación**: ábrelo igualmente, salta los
  paquetes ilegibles y dilo cuántos.
- Un paquete sin decodificar **no desaparece**: aparece con sus bytes.
- Ningún mensaje técnico en crudo. Traduce el error a algo que una persona
  entienda sin contexto.

---

## Accesibilidad y textos

- Todo en español y en segunda persona («No hay paquetes», no «sin datos»).
- Atajos de teclado para lo frecuente: abrir, capturar, parar, siguiente/anterior
  paquete, saltar al filtro.
- No dependas solo del color: el estado de error también lleva icono o texto.
- Contraste suficiente en fondo oscuro y en fondo claro.

---

## egui en concreto

Decisiones ya tomadas (ADR-0004): `egui` + `eframe`, y `egui` es dependencia
**solo** de este crate, activada por el feature `gui`.

- `CentralPanel` para la lista, `SidePanel` para el detalle, `TopBottomPanel`
  para las barras. Es el esqueleto estándar de egui.
- La API de egui cambia entre versiones: **fija la versión en `Cargo.toml` y
  encapsúlala en `views/`**. Nada fuera de ese directorio debería importar
  `egui` directamente.
- Nada de lógica dentro del closure de pintado. Si un closure de 20 líneas
  calcula algo, ese cálculo va en `state.rs`.

---

## Criterios de aceptación

- [ ] La lógica se puede probar sin abrir una ventana.
- [ ] 10.000 paquetes se listan a 60 fps.
- [ ] Un `.pcap` de 100 MB se abre mostrando progreso, sin bloquearse.
- [ ] Los paquetes con error se distinguen de un vistazo.
- [ ] El clic en un campo resalta su rango en la vista de bytes.
- [ ] Ningún `match` sobre protocolos fuera de una tabla de colores.

---

## Lo primero que haces mañana

1. Leer `docs/CONTRATOS.md` y proposal cambios si la API de A no te encaja.
2. Escribir `state.rs` con sus pruebas, **sin tocar egui**. Es el primer
   entregable del Sprint 1 y es la base de todo lo demás.
3. El día 3, enseñar la maqueta con datos falsos: es cuando se detecta pronto si
   el modelo de B no sirve para lo que hay que mostrar.