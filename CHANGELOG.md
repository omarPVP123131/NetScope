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
- Definición de alcance, con lo que entra, lo que no y qué significa «terminado»
  (`docs/ALCANCE.md`).
- Roadmap de 12 sprints para tres personas independientes (`docs/ROADMAP.md`).
- Guía de anonimización de capturas de tráfico propio (`fixtures/README.md`).
- Revisión crítica de la planificación: huecos, contradicciones y riesgos reales
  (`docs/REVISION.md`).
- Decisión cerrada de la lectura de PCAP y PCAPNG, con datos de las bibliotecas
  candidatas (ADR-0006).
- Licencia doble MIT / Apache-2.0 con los textos canónicos.
- Convenciones de código: identificadores en inglés y documentación en español.
- Calendario hasta abril de 2027, con las fiestas tenidas en cuenta.
- Plantillas de issues y PR, y código de conducta, para el repositorio.
- Registros de decisión arquitectónica 0001 a 0006 (`docs/adr/`).
- Notas de estudio de protocolos (`docs/PROTOCOLOS.md`).
- Guía de contribución, política de seguridad y exclusiones de ficheros.
- Estructura de carpetas de los tres crates y de las zonas de fixtures.

### Cambiado

- Los crates se renombran antes de escribir la primera línea de código:
  `netscope-proto` → `netscope-protocol`, `netscope-core` → `netscope-capture`,
  `netscope-ui` → `netscope-desktop`. El criterio y las alternativas están en el
  ADR-0001. El binario sigue llamándose `netscope`.

### Cambiado

- **El alcance se redefine**: el objetivo pasa a ser «las mismas categorías de
  función que Wireshark sobre menos protocolos» (`docs/ALCANCE.md`), con una
  lista explícita de lo que no entra.
- **El roadmap crece de 8 a 12 sprints** (19 oct 2026 – 9 abr 2027), con
  capacidad real de 15-20 h semanales por persona.
- Se sustituye el *fuzzing* por un **corpus de tramas mutadas a mano** como
  entrada hostil: mismos beneficios, sin dependencias.

### Añadido

- ADR-0007: distribución y privilegios. Binario portátil siempre disponible e
  instalador opcional que concede permisos de captura con *capabilities* en Linux
  y detección de Npcap en Windows.
- Validación cruzada con Wireshark: cada protocolo se contrasta con `tshark`, a
  mano al terminarlo y automáticamente en CI.
- Bocetos de la interfaz, con lo que cada pantalla exige al modelo de datos.
- Backlog detallado del Sprint 1 con puntos, dependencias y camino crítico.
- Hoja de firmas imprimible del contrato.
- Plantillas de las pruebas de contrato entre equipos.

### Cambiado

- La captura en vivo se limita a **Linux y Windows**. macOS abre ficheros pero
  no captura (ADR-0005 y ADR-0007).

### Nota

El proyecto está **en planificación**: el workspace compila y las comprobaciones
pasan, pero el analizador no hace todavía nada. No hay ninguna funcionalidad
del producto implementada.