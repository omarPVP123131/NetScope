# ADR-0001 · Workspace de tres crates

**Estado:** propuesta · **Fecha:** Sprint 1 · **Personas afectadas:** las tres

## Contexto

Tres personas van a trabajar a la vez. La primera pregunta no es *qué tecnología
usamos*, sino *qué forma hace que nadie bloquee a nadie*. Un único crate con
módulos compila todo siempre: cualquier cambio de un protocolo obliga a
recompilar y revalidar el analizador completo, y los conflictos de código se
multiplican.

## Decisión

El proyecto será un **workspace de Cargo con tres crates** y dependencias
estrictamente lineales:

```text
netscope-protocol  ◄──  netscope-capture  ◄──  netscope-desktop
```

| Crate | Dueño | Responsabilidad |
|-------|-------|-----------------|
| `netscope-protocol` | B | Modelo de datos y decodificadores. Sin dependencias internas |
| `netscope-capture` | B depende de A | Fuentes de datos, almacén, filtros, estadísticas |
| `netscope-desktop` | C | Interfaz. Depende de las dos anteriores |

Cada persona es autora de un crate entero. Nadie edita ficheros del crate de
otro.

## Sobre los nombres

Los tres nombres dicen **qué hace** el crate, no qué capa técnica ocupa:

- `netscope-protocol` — qué significan los bytes. `protocolo` en lugar de `proto`
  porque es una palabra completa, se lee bien en un import y no se confunde con
  el `proto` de otra cosa.
- `netscope-capture` — de dónde salen los paquetes. `capture` cubre tanto leer
  un fichero como escuchar la interfaz, que es justo lo que hace; `core` habría
  sido un nombre que no dice nada.
- `netscope-desktop` — la aplicación de escritorio. `ui` era demasiado ambiguo:
  en un proyecto con esta forma, `ui` puede ser el crate entero o solo la capa
  de presentación.

El nombre del **binario** es `netscope`, sin sufijo: es lo que el usuario
escribe. Los sufijos solo existen para el interior del workspace.

## Consecuencias

**A favor**

- B no recompila la interfaz; A no recompila los decodificadores.
- Los conflictos de git son improbables: cada persona trabaja en su carpeta.
- El paralelismo es real y no por cortesía.
- El DAG documenta la arquitectura: se lee el `Cargo.toml` y ya sabes qué sabe
  cada parte.

**En contra**

- Más `Cargo.toml` que mantener.
- Tipos compartidos que se atraviesen (por ejemplo, la configuración) exigen
  decidir en qué crate viven. Regla: en `protocol`, que es el que nadie más puede
  modificar sin romper a los demás.
- Antes de la primera línea hay que escribir los contratos
  ([`CONTRATOS.md`](../CONTRATOS.md)). Es trabajo previo, pero es el trabajo que
  ahorra los bloqueos.

## Alternativas descartadas

- **Un crate con módulos**: simple al principio, imposible de paralelizar.
- **Dos crates** (`capture` + `desktop`): mezcla "qué significan los bytes" con "de
  dónde vienen", y B acabaría bloqueado por la captura en vivo.
- **Cuatro crates** (`cli` aparte): el CLI se extrae en el Sprint 3 si hace
  falta, sin tocar esta decisión.