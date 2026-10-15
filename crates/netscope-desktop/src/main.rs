//! Binario `netscope`.
//!
//! **Dueño: Persona C.**
//!
//! Por ahora solo arranca y avisa de que el proyecto está en construcción: la
//! ventana entra en el Sprint 1, a partir de la maqueta de tres paneles
//! (lista, detalle, bytes) descrita en el roadmap.

// En Windows, sin esto saldría una consola de depuración delante de la ventana.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // TODO(Sprint 1, Persona C): sustituir por el arranque de egui con la
    // ventana real. Ver crates/netscope-desktop/README.md para el árbol previsto.
    println!("NetScope — el proyecto está en planificación.");
    println!("Sin interfaz todavía: esto es el esqueleto del Sprint 1.");
}
