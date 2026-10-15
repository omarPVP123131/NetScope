# NetScope

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
- [Alcance por fases](#alcance-por-fases)
- [Arquitectura](#arquitectura)
- [Reparto del trabajo: tres frentes independientes](#reparto-del-trabajo-tres-frentes-independientes)
- [Roadmap](#roadmap)
- [Tecnologías](#tecnologías)
- [Estructura prevista del repositorio](#estructura-prevista-del-repositorio)
- [Puesta en marcha](#puesta-en-marcha)
- [Seguridad y uso responsable](#seguridad-y-uso-responsable)
- [Documentación](#documentación)
- [Estado y licencia](#estado-y-licencia)

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
- **No es multiplataforma desde el primer día.** La captura en vivo se prioriza en
  Linux y Windows (ver [ADR-0005](docs/adr/0005-captura-en-vivo.md)); el resto del
  proyecto sí debe ser multiplataforma desde el principio.
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

Al terminar, una persona que no conoce el código debe poder:

- abrir un `.pcap`, elegir un paquete y ver sus campos con offset y valor
  hexadecimal;
- entender por qué un paquete aparece marcado como *truncado* o *malformado*;
- filtrar la captura por protocolo, dirección y puerto;
- añadir un protocolo nuevo (por ejemplo, DHCP) **sin tocar nada más que dos
  archivos**.

---

## Alcance por fases

| Fase | Contenido | Entregable verificable | Sprint |
|------|-----------|------------------------|--------|
| **1. Fundamentos** | Diseño de arquitectura, contratos entre equipos, Glosario de protocolos, capturas de prueba | Documentos de arquitectura y contratos firmados; repo con estructura de crates | 1 |
| **2. Parser de fichero** | Lectura de PCAP, Ethernet, IPv4, IPv6, TCP, UDP, ARP; pruebas unitarias con tramas truncadas y malformadas | CLI que lista y volca un `.pcap` | 2-3 |
| **3. Interfaz gráfica** | Lista de paquetes, panel de detalle, vista de bytes, filtros, búsqueda | Aplicación de escritorio que abre un `.pcap` | 4-5 |
| **4. Captura en vivo** | Integración con la biblioteca de captura del sistema, sin bloquear la interfaz, controles de inicio/parada | Botón "capturar" que muestra tráfico real | 6 |
| **5. Avanzado** | Reensamblado de flujos TCP, estadísticas, export, generador de paquetes de laboratorio | Informe de flujo y pcap exportado | 7-8 |

El detalle semana a semana está en [`docs/ROADMAP.md`](docs/ROADMAP.md).

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

8 sprints de 2 semanas (16 semanas), del 1 al 8.

| Sprint | Foco | Persona A | Persona B | Persona C |
|--------|------|-----------|-----------|-----------|
| 1 | Contratos y cimientos | Workspace de Cargo, ADR de captura | Modelo `Packet`/`Layer`/`Field` y esqueleto de decodificadores | Maqueta de la interfaz y navegación |
| 2 | Leer y entender | Lector PCAP con validación | Ethernet + IPv4 + UDP con tests | Lista de paquetes y columnas |
| 3 | Casos límite | PCAPNG, replay, manejo de ficheros corruptos | TCP + ARP + ICMP + errores y truncados | Panel de detalle con offsets |
| 4 | Mostrar los bytes | Almacén con capacidad y estadísticas | IPv6, flags, checksum, fuzzing del parser | Vista hexadecimal y resaltado |
| 5 | Usable | Filtros y búsqueda | DNS, DHCP, pulido y pruebas doradas | Filtros en interfaz y búsqueda |
| 6 | Tiempo real | Captura en vivo (Linux/Windows) y estadísticas en vivo | Rendimiento del decoder + reensamblado TCP | Controles de captura y stats |
| 7 | Laboratorio | Generador de paquetes | Reensamblado y pruebas de flujo | Export + panel de flujo |
| 8 | Cierre | Empaquetado y publicación | Endurecimiento final y documentación | Pulido, onboarding y ayuda en línea |

Detalle, dependencias, riesgos y rituales: [`docs/ROADMAP.md`](docs/ROADMAP.md).

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
git clone https://github.com/netscope-project/netscope
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

La **captura en vivo** (Sprint 6) sí añadirá requisitos del sistema: `libpcap`
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
| [`docs/SPRINT-1.md`](docs/SPRINT-1.md) | El primer sprint, día a día, con las tareas de cada persona |
| [`docs/ARQUITECTURA.md`](docs/ARQUITECTURA.md) | Decisiones de diseño, flujo de datos, concurrencia, pruebas |
| [`docs/CONTRATOS.md`](docs/CONTRATOS.md) | Qué puede usar cada equipo del otro, y qué **no** puede |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Qué hace cada persona cada semana, con dependencias y riesgos |
| [`docs/PROTOCOLOS.md`](docs/PROTOCOLOS.md) | Notas de estudio: qué hay que mirar en cada protocolo antes de escribirlo |
| [`docs/personas/`](docs/personas/) | **La guía de cada persona**: qué construye, cómo, con qué trampas y con qué criterios de aceptación |
| [`docs/adr/`](docs/adr/) | Registros de decisiones arquitectónicas (ADR) |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Cómo se trabaja en el repositorio: ramas, commits, PR, revisiones |
| [`SECURITY.md`](SECURITY.md) | Uso autorizado, datos sensibles y reporte de fallos de seguridad |

---

## Estado y licencia

**Estado:** planificación. El roadmap está definido y el reparto de trabajo
acordado; la implementación empieza en el Sprint 1.

**Licencia:** se definirá antes de la primera publicación pública del
proyecto. Hasta entonces el repositorio es privado y no se distribuye.

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