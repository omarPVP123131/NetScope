# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y
[versionado semántico](https://semver.org/lang/es/).

## [No publicado]

### Añadido

- README con alcance, arquitectura, reparto del trabajo y roadmap.
- Workspace de Cargo con los tres crates, `lints` compartidos y `Cargo.lock`
  versionado. Compila, y pasa `fmt`, `clippy` y `test`.
- Esqueleto mínimo: el `lib.rs` de cada crate y el binario `netscope`, que
  arranca y avisa de que el proyecto está en construcción.
- Integración continua: `fmt`, `clippy` y `test` en cada commit, compilación de
  la interfaz aparte y matriz de Linux, Windows y macOS
  (`.github/workflows/ci.yml`).
- Features declarados para lo que vendrá: captura por sistema operativo
  (`capture-linux`, `capture-windows`, `capture-macos`) y lectura de `pcapng`.
- Arquitectura del proyecto: workspace de tres crates con DAG lineal
  (`docs/ARQUITECTURA.md`).
- Contratos entre equipos con reglas de dependencia y pruebas de contrato
  (`docs/CONTRATOS.md`).
- Roadmap de 8 sprints para tres personas independientes (`docs/ROADMAP.md`).
- Registros de decisión arquitectónica 0001 a 0006 (`docs/adr/`).
- Notas de estudio de protocolos (`docs/PROTOCOLOS.md`).
- Guía de contribución, política de seguridad y exclusiones de ficheros.
- Estructura de carpetas de los tres crates y de las zonas de fixtures.

### Cambiado

- Los crates se renombran antes de escribir la primera línea de código:
  `netscope-proto` → `netscope-protocol`, `netscope-core` → `netscope-capture`,
  `netscope-ui` → `netscope-desktop`. El criterio y las alternativas están en el
  ADR-0001. El binario sigue llamándose `netscope`.

### Nota

El proyecto está **en planificación**: el workspace compila y las comprobaciones
pasan, pero el analizador no hace todavía nada. No hay ninguna funcionalidad
del producto implementada.