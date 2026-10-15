# NetScope

[![CI](https://github.com/omarPVP123131/NetScope/actions/workflows/ci.yml/badge.svg)](https://github.com/omarPVP123131/NetScope/actions/workflows/ci.yml)
[![Licencia](https://img.shields.io/badge/licencia-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE.md)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Estado](https://img.shields.io/badge/estado-planificaci%C3%B3n-amber.svg)](docs/ROADMAP.md)
[![Contribuyendo](https://img.shields.io/badge/contribuciones-bienvenidas-blue.svg)](CONTRIBUTING.md)

**Analizador de paquetes de red escrito en Rust, con fines educativos.**

NetScope existe para entender *de verdad* cómo se estructuran los paquetes: no
solo qué byte va en qué sitio, sino por qué está ahí, qué se rompe cuando el
byte no es el esperado y qué tiene que hacer un analizador para no caer cuando
el fichero que le das está roto.

> **Estado del proyecto: en planificación.** Ninguna de las funcionalidades
> descritas está implementada todavía. Este documento es un plan de trabajo, no
> una descripción de software existente.

---

## Índice

- [Qué es y qué no es](#qué-es-y-qué-no-es)
- [Objetivos](#objetivos)
- [Alcance](#alcance)
- [Fases](#alcance-por-fases)
- [Arquitectura](#arquitectura)
- [Reparto del trabajo: tres frentes independientes](#reparto-del-trabajo-tres-frentes-independientes)
- [Roadmap](#roadmap)
- [Tecnologías](#tecnologías)
- [Estructura prevista del repositorio](#estructura-prevista-del-repositorio)
- [Puesta en marcha](#puesta-en-marcha)
- [Seguridad y uso responsable](#seguridad-y-uso-responsable)
- [Documentación](#documentación)
- [Estado](#estado)
- [Licencia](#licencia)

---

## Qué es y qué no es

### Qué es

- Un analizador de paquetes **de escritorio**, en Rust, con interfaz gráfica.
- Un lector de capturas **en fichero** (PCAP y PCAPNG) y, más adelante, en
  **tiempo real** sobre la interfaz de red del sistema.
- Un decodificador de las cabeceras de Ethernet, ARP, IPv4, IPv6, TCP, UDP,
  ICMP y DNS, con el detalle campo a campo: nombre, valor, offset y bytes.
- Un proyecto de aprendizaje con un parser pensado para **no confiar en sus
  entradas**.

### Qué no es (y no será)

- **No es una herramienta de ataque.** No genera escáneres, no realiza
  ataques, no evade detección. El generador de paquetes existe para montar
  redes de laboratorio controladas y comprobar el propio analizador.
- **No sustituye a Wireshark.** Es un ejercicio didáctico con un alcance
  deliberadamente acotado. Lo que no se implemente, se documenta como pendiente.
- **La captura en vivo solo funciona en Linux y Windows.** En macOS el programa
  abre ficheros pero no captura, y lo dice con un mensaje claro
  ([ADR-0005](docs/adr/0005-captura-en-vivo.md) y
  [ADR-0007](docs/adr/0007-distribucion-y-privilegios.md)). El resto del
  proyecto sí compila y funciona en los tres sistemas desde el principio.
- **No expone una API de red.** No envía ni recibe tráfico; solo lo muestra.

---

## Objetivos

1. **Comprender los protocolos analizando paquetes reales**, no ejemplos de
   libro: ficheros de captura reales y malformados incluidos.
2. **Practicar programación de sistemas en Rust**: `ownership`, `borrowing`,
   slices, enums, `Result`, concurrencia y ausencia de `unsafe`.
3. **Practicar representación binaria**: offsets, endianness, máscaras de bits,
   sumas de verificación y validación de longitudes.
4. **Construir una herramienta útil y legible** que otra persona pueda leer de
   arriba abajo y entender por qué cada decisión está tomada.

### Criterio de éxito del proyecto

**No** es el número de protocolos ni parecerse a Wireshark por fuera. Es uno:

> Alguien que no ha escrito el código abre una captura de tráfico real,
> encuentra lo que busca usando filtros, sigue una conexión TCP de principio a
> fin y exporta lo que le interesa, **sin leer una línea del código**.

Si eso pasa con ocho protocolos, el proyecto ha funcionado. Si no pasa con
quince, no ha funcionado.

---

## Alcance

**El objetivo es tener las mismas categorías de función que Wireshark sobre un
conjunto de protocolos mucho más pequeño.** Abrir capturas, ver los campos con
su offset, ver los bytes, filtrar, seguir flujos, capturar en vivo, estadísticas
y exportar. La misma amplitud de funciones, no la misma profundidad ni la misma
cobertura de protocolos: Wireshark entiende más de dos mil porque son veinte
años y ciento cincuenta personas.

Lo que **no** entra, y por qué, está escrito en
[`docs/ALCANCE.md`](docs/ALCANCE.md). Esa lista es la que más tiempo ahorra:
añadir un protocolo número trece antes de terminar el número cuatro es cómo
mueren estos proyectos.

## Fases

| Fase | Contenido | Entregable verificable | Sprints |
|------|-----------|------------------------|---------|
| **1. Fundamentos** | Arquitectura, contratos entre equipos, modelo de datos, capturas de prueba | Contratos firmados y workspace que compila | 1 |
| **2. Parser de fichero** | PCAP, PCAPNG, Ethernet, IPv4, IPv6, TCP, UDP, ARP, ICMP, DNS, DHCP, VLAN | Un `.pcap` se abre y se ve campo a campo | 2-6 |
| **3. Interfaz gráfica** | Lista, detalle, vista de bytes, filtros, búsqueda, estadísticas | Aplicación de escritorio usable por alguien externo | 2-7 |
| **4. Flujos y laboratorio** | Reensamblado TCP, conversaciones, generador de paquetes | Una conexión TCP completa, de principio a fin | 8-9 |
| **5. Captura en vivo** | Integración con el sistema, sin bloquear la interfaz, estadísticas en vivo | Botón «capturar» que muestra tráfico real | 10-11 |
| **6. Cierre** | Rendimiento, binarios portátiles por sistema, documentación | v0.1 publicada y usada por alguien fuera del equipo | 12 |

El detalle está en [`docs/ROADMAP.md`](docs/ROADMAP.md) y el alcance, en
[`docs/ALCANCE.md`](docs/ALCANCE.md).

---

## Arquitectura

El proyecto es un **workspace de Cargo con tres crates** y un DAG estrictamente
lineal. Esa forma no es estética: es lo que permite que tres personas
trabajen a la vez sin bloquearse.

```
netscope-protocol  ──►  netscope-capture  ──►  netscope-desktop
modelo y decoders       fuentes y almacén     interfaz gráfica
```

- **Nadie depende hacia atrás.** `protocol` no sabe que existe una interfaz;
  `capture` no sabe que existe un sistema operativo concreto; `desktop` no sabe qué
  fichero se está leyendo.
- **El contrato público se congela en el Sprint 1.** A partir de ahí, cada
  equipo puede trabajar contra firmas fijadas sin esperar al trabajo del otro.
- **Cambiar un contrato es un acto explícito**: ADR + versión + aviso, nunca un
  cambio silencioso que rompa a los demás.

Explicación completa: [`docs/ARQUITECTURA.md`](docs/ARQUITECTURA.md).
Firmas concretas y reglas de dependencia: [`docs/CONTRATOS.md`](docs/CONTRATOS.md).

### Capas y responsabilidades

| Crate | Es responsable de | Nunca hará |
|-------|-------------------|-------------|
| `netscope-protocol` | Modelo canónico (`Packet`, `Layer`, `Field`), registro de decodificadores, volcado hexadecimal | Abrir ficheros, dibujar, tocar la red |
| `netscope-capture` | Lectura de PCAP/PCAPNG, captura en vivo, almacén, filtros, estadísticas, generador de laboratorio | Interpretar campos de protocolo, dibujar |
| `netscope-desktop` | Lista, panel de detalle, vista de bytes, filtros, estadísticas, export | Interpretar bytes crudos, leer ficheros |

---

## Reparto del trabajo: tres frentes independientes

Cada persona es **dueña de un crate entero**. Nadie comparte un archivo con
nadie, y las dependencias entre equipos se cruzan por contratos escritos, no
por código compartido.

| Persona | Frente | Crate | Entregable de referencia | No depende de |
|---------|--------|-------|--------------------------|----------------|
| **A — Captura e ingesta** | Cómo entran los bytes al programa | `netscope-capture` | Lectura de `.pcap`, almacén con capacidad limitada, filtros, estadísticas | De que exista un decodificador de TCP |
| **B — Protocolos** | Qué significan los bytes | `netscope-protocol` | Decodificadores Ethernet/IPv4/IPv6/TCP/UDP/ARP, errores y casos límite | De que exista captura en vivo |
| **C — Interfaz** | Cómo se ve y se usa | `netscope-desktop` | Lista, detalle, vista hexadecimal, filtros, export | De que exista captura en vivo |

**Regla de oro:** si el trabajo de una persona bloquea a otra, es un error de
diseño, no una realidad del proyecto. La solución es un contrato o un doble de
prueba (*mock/fixture*), nunca esperar.

La guía completa de cada persona está en [`docs/personas/`](docs/personas/).

Herramientas que hacen posible la independencia:

- **Contratos escritos** antes de la primera línea de código
  ([`docs/CONTRATOS.md`](docs/CONTRATOS.md)).
- **Fixtures compartidas**: quien produce capturas de prueba se encarga de que
  existan desde el Sprint 1, para que los tres equipos tengan datos que mirar.
- **Pruebas de contrato**: cada equipo prueba su parte con las fixtures del
  otro, nunca con la implementación del otro.
- **Integración por fusionar, no por esperar**: se integra al final de cada
  sprint, no cuando "el otro esté listo".

---

## Roadmap

**12 sprints de dos semanas: del 19 de octubre de 2026 al 9 de abril de 2027.**

El objetivo es tener **las mismas categorías de función que Wireshark** sobre un
conjunto de protocolos mucho más pequeño: abrir capturas, ver los campos, ver
los bytes, filtrar, seguir flujos, capturar en vivo, estadísticas y exportar. No
la misma cobertura: ver [`docs/ALCANCE.md`](docs/ALCANCE.md).

| Sprint | Foco | Persona A | Persona B | Persona C |
|--------|------|-----------|-----------|-----------|
| 1 | Contratos y cimientos | Fixtures, almacén, filtros | Modelo, registro, decoders base | `state.rs` y maqueta |
| 2 | Abrir un `.pcap` | Lector PCAP | Ethernet + IPv4 + UDP | Lista de paquetes |
| 3 | TCP y detalle | Rendimiento y búsqueda | TCP, ARP, ICMP | Detalle y vista de bytes |
| 4 | PCAPNG y límites | PCAPNG, exportación | IPv6 + extensiones | Estadísticas, búsqueda |
| 5 | 🎄 Navidades | Filtros `&&` `||` `!` | DNS, DHCP | Filtros en la interfaz |
| 6 | VLAN, ICMPv6 | Métricas por conversación | VLAN, ICMPv6 | Colores y filtros por MAC |
| 7 | Filtros y export | Filtros guardados, CSV/JSON | Estadísticas por flujo | Panel de estadísticas |
| 8 | Reensamblado TCP | Soporte de secuencias | Reordenamiento y retransmisiones | Vista de conexión |
| 9 | Flujos y laboratorio | Generador de paquetes | Validar el generador | Panel del generador |
| 10 | Captura en vivo | Spike y backend de Linux | Decodificación en segundo plano | Controles de captura |
| 11 | Windows y macOS | Backends y empaquetado | Protocolos de estiramiento | Estadísticas en vivo |
| 12 | Cierre | Binarios portátiles | Revisión de seguridad | Pulido y publicación |

Detalle día a día, dependencias, riesgos y rituales:
[`docs/ROADMAP.md`](docs/ROADMAP.md). El primer sprint, expandido:
[`docs/SPRINT-1.md`](docs/SPRINT-1.md).

---

## Tecnologías

| Tecnología | Propósito |
|-----------|-----------|
| **Rust** | Lenguaje principal del proyecto |
| **Cargo** | Gestión de dependencias y compilación |
| **PCAP / PCAPNG** | Formato de almacenamiento de capturas |
| **`pcap-parser`** | Lectura de ficheros PCAP (candidato a evaluar en el Sprint 2) |
| **`eframe` / `egui`** | Interfaz gráfica propuesta |
| **Buffers propios + `hexdump` propio** | Visualización de datos binarios, sin dependencias para lo básico |
| **`tshark` (Wireshark)** | Contrastar cada protocolo decodificado con la herramienta de referencia |
| **Git y GitHub** | Control de versiones, revisiones e incidencias |

Decisiones ya tomadas: [`docs/adr/`](docs/adr/). Decisiones pendientes:
librería de captura en vivo (Sprint 6) y formato de intercambio de fixtures.

---

## Estructura prevista del repositorio

```text
netscope/
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── SECURITY.md
├── Cargo.toml                      # workspace: un solo lugar para las versiones
├── Cargo.lock                      # versionado: esto es una aplicación, no una librería
├── .github/workflows/ci.yml        # fmt · clippy · test · matriz de 3 sistemas
├── crates/
│   ├── netscope-protocol/          # Persona B — modelo y decodificadores
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── lib.rs  model.rs  decoder.rs  registry.rs  error.rs  hexdump.rs
│   │       └── layers/             # un archivo por protocolo
│   ├── netscope-capture/           # Persona A — lectura, almacén, filtros
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── lib.rs  source.rs  pipeline.rs  store.rs  filter.rs  stats.rs
│   │       ├── pcap/               # file.rs (Sprint 2), pcapng.rs (Sprint 3)
│   │       ├── capture/            # único código por sistema (Sprint 6)
│   │       └── generator.rs        # laboratorio (Sprint 7)
│   └── netscope-desktop/           # Persona C — interfaz
│       ├── Cargo.toml
│       ├── README.md
│       └── src/
│           ├── lib.rs  main.rs  state.rs  app.rs
│           └── views/              # lista, detalle, bytes, filtros, stats
├── docs/
│   ├── personas/                   # la guía de cada persona
│   │   ├── PERSONA-A-captura.md
│   │   ├── PERSONA-B-protocolos.md
│   │   └── PERSONA-C-interfaz.md
│   ├── SPRINT-1.md
│   ├── ARQUITECTURA.md
│   ├── CONTRATOS.md
│   ├── ROADMAP.md
│   ├── PROTOCOLOS.md
│   └── adr/                        # 0001 a 0006
├── fixtures/
│   ├── README.md
│   ├── minimos/                    # tramas escritas a mano: lo primero que hay que escribir
│   ├── reales/                     # capturas propias, anonimizadas, < 1 MB
│   ├── dorados/                    # resultado esperado de cada trama
│   └── carga/                      # ficheros grandes para medir rendimiento
```

El esqueleto ya existe y compila; los archivos que no están todavía son los que
escribís en los sprints. El `Cargo.toml` de cada crate lleva su dueño en un
comentario y los `lints` heredados del workspace, así que las reglas
(`unsafe` prohibido, documentación obligatoria) se aplican a los tres por igual
sin repetir configuración.

---

## Puesta en marcha

El workspace **compila y pasa las comprobaciones** desde ya mismo: lo que aún no
existe es el analizador. El binario arranca y avisa de que todavía no hay
interfaz.

Requisitos:

- **Rust estable** (edition 2024 o posterior) con `rustfmt` y `clippy`.
- Nada más. Leer ficheros no necesita instalar nada.

```bash
git clone https://github.com/omarPVP123131/NetScope
cd netscope

cargo check --workspace --no-default-features    # rápido
cargo test  --workspace --no-default-features
cargo clippy --workspace --all-targets --no-default-features -- -D warnings

cargo run -p netscope-desktop                    # el binario `netscope`
```

### Qué es eso de `--no-default-features`

La interfaz gráfica es el único motivo por el que el workspace necesita
dependencias pesadas. Ese flag comprueba y prueba **todo lo demás** sin
descargarlas, y es lo que hace integración continua en cada commit:

| Comando | Qué incluye | Cuándo se usa |
|---------|-------------|---------------|
| `cargo check --workspace --no-default-features` | Los tres crates, sin interfaz | Cada commit |
| `cargo check -p netscope-desktop --all-features` | También la interfaz gráfica | Antes de integrar la interfaz |
| `cargo run -p netscope-desktop` | Arranca la aplicación | Probar la ventana |

En cuanto la ventana exista (Sprint 1), `cargo run -p netscope-desktop` abrirá la
aplicación de verdad.

### Cómo se distribuye

Binarios **portátiles** en la sección de *releases*, uno por sistema, sin
instalar nada: se descarga, se descomprime y se ejecuta. Lo genera integración
continua al publicar una etiqueta (`v0.1.0`), no a mano.

Solo la captura en vivo necesita algo del sistema (`libpcap` en Linux, Npcap en
Windows). Para **abrir ficheros no hace falta instalar nada**.

La **captura en vivo** (Sprints 10-11) sí añadirá requisitos del sistema: `libpcap`
en Linux o Npcap en Windows. Hasta entonces no hace falta nada.

---

## Seguridad y uso responsable

NetScope está destinado al **aprendizaje, la investigación y el diagnóstico
autorizado** de redes.

- **La captura en vivo solo en redes y dispositivos donde exista autorización
  explícita.** Capturar el tráfico de terceros sin permiso es ilegal en
  prácticamente cualquier jurisdicción. En muchas redes, el simple hecho de
  instalar la herramienta requiere privilegios de administrador.
- **El generador de paquetes se usa en laboratorio.** Sirve para montar redes de
  prueba controladas y verificar el analizador, no para generar tráfico real.
- **Los ficheros de captura son datos sensibles.** Un `.pcap` puede contener
  contraseñas en texto claro, cabeceras de autenticación, datos personales o
  información de terceros. No se suben a issue ni se adjuntan a PR sin
  revisar, y el repositorio incluye una lista de exclusión para `*.pcap`.
- **El parser trata toda entrada como hostil.** Un fichero de captura es datos no
  confiables: longitudes absurdas, offsets fuera de rango, cabeceras de longitud
  variable y campos que apuntan más allá del buffer. El diseño no permite leer
  fuera de rango ni confiar en un valor leído de un fichero para reservar
  memoria o indexar.
- **Nada de sniffing activo, escaneo ni evasión.** Queda fuera de alcance por
  decisión del proyecto.

Si encuentras un fallo de seguridad en el parser, no abras un issue público:
sigue el proceso de reporte confidencial descrito en
[`SECURITY.md`](SECURITY.md).

---

## Documentación

| Documento | Para qué sirve |
|-----------|----------------|
| [`docs/MOCKUPS.md`](docs/MOCKUPS.md) | Bocetos de las pantallas: qué se ve en cada zona y qué exige al modelo |
| [`docs/VALIDACION-WIRESHARK.md`](docs/VALIDACION-WIRESHARK.md) | Cómo se comprueba que cada protocolo se interpreta como Wireshark |
| [`docs/SPRINT-1-BACKLOG.md`](docs/SPRINT-1-BACKLOG.md) | El Sprint 1 tarea a tarea, con puntos y dependencias |
| [`docs/CONTRATOS-FIRMA.md`](docs/CONTRATOS-FIRMA.md) | Hoja imprimible para firmar el contrato el primer día |
| [`docs/PLANTILLAS-PRUEBAS-CONTRATO.md`](docs/PLANTILLAS-PRUEBAS-CONTRATO.md) | Plantillas de las pruebas cruzadas entre equipos |
| [`docs/ALCANCE.md`](docs/ALCANCE.md) | **Qué es y qué no es NetScope**: la definición de «comparable a Wireshark» y la lista de lo que no entra |
| [`docs/SPRINT-1.md`](docs/SPRINT-1.md) | El primer sprint, día a día, con las tareas de cada persona |
| [`docs/ARQUITECTURA.md`](docs/ARQUITECTURA.md) | Decisiones de diseño, flujo de datos, concurrencia, pruebas |
| [`docs/CONTRATOS.md`](docs/CONTRATOS.md) | Qué puede usar cada equipo del otro, y qué **no** puede |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Qué hace cada persona cada semana, con dependencias y riesgos |
| [`docs/PROTOCOLOS.md`](docs/PROTOCOLOS.md) | Notas de estudio: qué hay que mirar en cada protocolo antes de escribirlo |
| [`docs/personas/`](docs/personas/) | **La guía de cada persona**: qué construye, cómo, con qué trampas y con qué criterios de aceptación |
| [`docs/adr/`](docs/adr/) | Registros de decisiones arquitectónicas (ADR) |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Cómo se trabaja en el repositorio: ramas, commits, PR, revisiones |
| [`SECURITY.md`](SECURITY.md) | Uso autorizado, datos sensibles y reporte de fallos de seguridad |
| [`docs/REVISION.md`](docs/REVISION.md) | Revisión crítica de la propia planificación: huecos y riesgos reales |
| [`LICENSE.md`](LICENSE.md) | Doble licencia MIT / Apache-2.0 y cuál elegir |

---

## Estado

**Planificación.** El workspace compila y pasa las comprobaciones de formato,
análisis estático y pruebas; el analizador todavía no hace nada. El roadmap
está definido, el reparto de trabajo acordado y la implementación empieza en el
Sprint 1.

Lo que ya existe: la arquitectura, los contratos entre equipos, el roadmap de
8 sprints, las guías de cada persona y el esqueleto del proyecto.
Lo que no: todo el producto. Véase [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Licencia

**MIT o Apache-2.0**, a elección de quien la use. Los dos textos están en el
repositorio y el detalle de cuál conviene en [`LICENSE.md`](LICENSE.md).

---

## Colaboración

El proyecto está pensado para desarrollarse en equipo. Los tres frentes
(captura, protocolos e interfaz) se pueden implementar y probar de forma
independiente.

Las contribuciones deben:

- incluir pruebas cuando toquen decodificadores, filtros o la captura;
- mantener la separación entre **leer datos**, **interpretar protocolos** y
  **mostrar resultados**;
- no romper un contrato documentado sin un ADR que lo acompañe y sin actualizar
  `docs/CONTRATOS.md`.

Lee [`CONTRIBUTING.md`](CONTRIBUTING.md) antes de abrir tu primer PR.