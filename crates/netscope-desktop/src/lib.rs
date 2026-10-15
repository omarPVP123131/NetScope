//! # netscope-desktop
//!
//! La ventana de NetScope y el binario `netscope`.
//!
//! **Dueño: Persona C.** El contenido lo escribe la Persona C en el Sprint 1.
//!
//! Responde a una sola pregunta: **¿cómo se ve lo que ya está interpretado?**
//! No lee ficheros, no habla con la red y no interpreta bytes crudos: todo lo
//! que se muestra llega ya en el modelo.
//!
//! ## Qué va aquí
//!
//! - `state`: el estado de la aplicación, **sin egui**, para poder probarlo sin
//!   abrir una ventana.
//! - La ventana: lista de paquetes, detalle campo a campo, vista de bytes,
//!   filtros y estadísticas.
//!
//! ## Reglas
//!
//! - Los campos se buscan **por nombre**, nunca por posición: `fields[3]` se
//!   rompe en cuanto alguien añade un campo a un protocolo.
//! - No hay `match` sobre protocolos en la presentación: el modelo ya viene
//!   interpretado.
//!
//! ## Contrato
//!
//! [`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md). Interfaz elegida:
//! [`../../docs/adr/0004-egui.md`](../../docs/adr/0004-egui.md).
