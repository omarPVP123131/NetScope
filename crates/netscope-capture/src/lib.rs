//! # netscope-capture
//!
//! Lectura de capturas, almacén de paquetes, filtros y estadísticas.
//!
//! **Dueño: Persona A.** El contenido lo escribe la Persona A en el Sprint 1.
//!
//! Responde a una sola pregunta: **¿de dónde salen los bytes y qué se hace con
//! ellos?** No sabe qué significa un número de secuencia TCP; pide al registro
//! de `netscope-protocol` que lo interprete y guarda el resultado.
//!
//! ## Qué va aquí
//!
//! - `PacketSource`: la abstracción común a leer un fichero y a escuchar la red.
//! - Lectores de PCAP y PCAPNG.
//! - `PacketStore`: el almacén con índice, capacidad limitada y consulta.
//! - Filtros por protocolo, dirección, puerto y texto libre.
//! - Estadísticas: totales, distribución por protocolo y caudal.
//! - El generador de paquetes de laboratorio (Sprint 7).
//!
//! ## Qué no va aquí
//!
//! Interpretación de campos de protocolo. Si este crate sabe qué es un TTL, los
//! filtros por protocolo dejan de ser genéricos y el filtro por bytes se
//! duplica en tres sitios.
//!
//! ## Reglas
//!
//! - Ningún fichero de captura, por hostil que sea, termina el programa.
//! - El almacén tiene tope de capacidad y lo dice en las estadísticas.
//! - Nunca se mantiene un bloqueo abierto mientras se decodifica o se captura.
//!
//! ## Contrato
//!
//! [`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md), sección
//! «De `capture` a `desktop`».
