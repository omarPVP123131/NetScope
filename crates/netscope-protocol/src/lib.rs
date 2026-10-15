//! # netscope-protocol
//!
//! Modelo canónico de paquetes y decodificadores de protocolo.
//!
//! **Dueño: Persona B.** El contenido lo escribe la Persona B en el Sprint 1.
//!
//! Este crate es la fuente de verdad del modelo de datos: qué es un paquete,
//! qué es una capa, qué es un campo y qué significa un error de decodificación.
//! No abre ficheros, no dibuja nada y no toca la red.
//!
//! ## Qué va aquí
//!
//! - El modelo: `Packet`, `Layer`, `Field`, `ByteRange`, `Protocol`, `LinkType`.
//! - El contrato de decodificación (`Decoder`) y el registro que asocia cada
//!   identificador de protocolo con su decodificador.
//! - Un archivo por protocolo en `layers/`.
//! - El volcado hexadecimal y ASCII.
//!
//! ## Qué no va aquí
//!
//! Lectura de ficheros, sistema operativo e interfaz. Si alguna de esas cosas
//! hace falta, es que el tipo está en el crate equivocado.
//!
//! ## Reglas
//!
//! - Los bytes de un fichero de captura **no son de fiar**. Toda lectura pasa
//!   por un lector con límites comprobados y devuelve un error en lugar de
//!   indexar fuera de rango.
//! - Los campos **no copian** los bytes: guardan un rango dentro de la trama.
//! - Un paquete con problemas sigue siendo un paquete: se marca, no se tira.
//! - `unsafe` está prohibido por el workspace (ver `Cargo.toml`).
//!
//! ## Contrato
//!
//! La superficie pública está descrita en
//! [`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md). Cambiarla requiere ADR.
