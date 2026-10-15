# ADR-0005 · Captura en vivo detrás de un trait, Linux y Windows primero

**Estado:** propuesta · **Fecha:** Sprint 1 · **Personas afectadas:** A

## Contexto

La captura en vivo necesita tocar el sistema operativo, y cada uno lo hace de
forma distinta:

| Sistema | Mecanismo | Dependencia |
|---------|-----------|-------------|
| Linux | `AF_PACKET` / `libpcap` | `libpcap` |
| Windows | Npcap (compatible con libpcap) | Npcap, permisos de administrador |
| macOS | `BPF` / libpcap | `libpcap`, permisos |

Hacerlo bien en los tres a la vez, desde el primer día, obliga a C a esperar a
que A tenga una API de captura que además funcione en su sistema, y complica
las pruebas de integración continua.

## Decisión

1. La captura en vivo se define **detrás de un trait** (`LiveCapture`), con el
   mismo contrato que la lectura de ficheros. El resto del programa no sabe si
   los paquetes vienen de un disco o del cable.
2. Se compila **por sistema**, con *features*: `capture-linux` y
   `capture-windows`. Si el sistema no está soportado, el programa arranca igual
   y explica por qué no puede capturar.
3. **Linux (Sprint 10) y Windows (Sprint 11)** son los sistemas con captura en
   vivo. **macOS queda fuera de la v0.1**: el programa abre ficheros allí, pero
   no captura, y lo dice con un mensaje claro. Ver
   [ADR-0007](0007-distribucion-y-privilegios.md).
4. Toda la integración con el sistema se queda en **un solo módulo** de
   `netscope-capture`. Es el único punto del proyecto que puede tener código
   específico del sistema operativo.
5. Antes de escribirlo, un *spike* de dos días compara al menos dos
   bibliotecas candidatas en términos de: multiplataforma, mantenimiento,
   licencia, comodidad de compilación en CI y consumo de memoria.

## Consecuencias

**A favor**

- C sigue trabajando contra `LiveCapture` aunque A no lo haya terminado: los
  sprints 10 y 11 de la interfaz no dependen de que la captura esté.
- El proyecto entero compila y se prueba en Linux, Windows y macOS desde el
  principio. Lo que no llega a macOS es la captura, no el programa.
- Se puede probar la captura en un entorno controlado (contenedor, máquina
  virtual) sin tocar la red real.

**En contra**

- El *spike* de dos días hay que hacerlo, o el Sprint 10 se convierte en una
  apuesta.
- **macOS se queda sin captura en vivo.** Es una limitación consciente: exige
  firma, *entitlement* y un instalador propio, que es justo lo que se decidió no
  hacer (ADR-0007).
- El Sprint 6 ya no es el de la captura: el de la captura en vivo son el 10 y el
  11. Las referencias al Sprint 6 de este documento son históricas.

## Alternativas descartadas

- **Una librería multiplataforma y ya está**: si esa librería usa `unsafe` y
  bloquea el hilo, arrastra sus problemas a todo el proyecto. Se revisa en el
  *spike*.
- **Captura en el hilo de la interfaz**: congelaría la ventana con tráfico de
  gigabitros.