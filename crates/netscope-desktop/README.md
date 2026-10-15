# netscope-desktop

**Dueño: Persona C (Interfaz y análisis).** Nadie más edita este crate.

## Qué vive aquí

La ventana y todo lo que se ve. También el binario `netscope`.

```text
src/
├── main.rs         # arranque de la aplicación
├── lib.rs
├── state.rs        # estado de la aplicación, sin egui: se prueba sin ventana
├── app.rs          # composición de la ventana
└── views/
    ├── lista.rs        # tabla de paquetes, columnas, selección
    ├── detalle.rs     # árbol de capas y campos con offset
    ├── bytes.rs       # vista hexadecimal y ASCII
    ├── filtros.rs     # editor de filtros y búsqueda
    ├── estadisticas.rs
    ├── flujo.rs       # reensamblado de flujos (Sprint 7)
    └── export.rs      # exportar selección
```

## Reglas

- **No** abrir ficheros ni llamar a la biblioteca de captura: se pide a
  `netscope-capture`.
- **No** interpretar bytes crudos: todo lo que se muestra ya viene en el modelo.
  No hay `match` sobre protocolos aquí.
- **No** buscar campos por posición (`fields[3]`): siempre por nombre
  (`layer.field("TTL")`), porque añadir un campo no debe romper la vista.
- La lógica de la aplicación vive en `state.rs` y **no** depende de `egui`, para
  poder probarla sin abrir una ventana.

## Contrato

[`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md). Decisión de interfaz:
[`../../docs/adr/0004-egui.md`](../../docs/adr/0004-egui.md).