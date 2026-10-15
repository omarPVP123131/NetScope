# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y
[versionado semántico](https://semver.org/lang/es/).

## [No publicado]

### Añadido

- README con alcance, arquitectura, reparto del trabajo y roadmap.
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

El proyecto está **en planificación**: no hay ninguna funcionalidad
implementada todavía.