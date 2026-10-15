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
2. Se compila **por sistema**, con *features*: `capture-linux`,
   `capture-windows`, `capture-macos`. Si el sistema no está soportado, el
   programa arranca igual y explica por qué no puede capturar.
3. **Linux y Windows son prioritarios** en el Sprint 6; **macOS entra si sobra
   tiempo** y no compromete el cierre.
4. Toda la integración con el sistema se queda en **un solo módulo** de
   `netscope-capture`. Es el único punto del proyecto que puede tener código
   específico del sistema operativo.
5. Antes de escribirlo, un *spike* de dos días compara al menos dos
   bibliotecas candidatas en términos de: multiplataforma, mantenimiento,
   licencia, comodidad de compilación en CI y consumo de memoria.

## Consecuencias

**A favor**

- C sigue trabajando contra `LiveCapture` aunque A no lo haya terminado: el
  Sprint 6 de la interfaz no depende del Sprint 6 de la captura.
- El resto del proyecto compila y se prueba en Windows, Linux y macOS desde el
  principio.
- Se puede probar la captura en un entorno controlado (contenedor, máquina
  virtual) sin tocar la red real.

**En contra**

- El SPIKE de dos días hay que hacerlo, o el Sprint 6 se convierte en una
  apuesta.
- macOS puede quedarse fuera. Se documenta como limitación consciente, no como
  sorpresa.

## Alternativas descartadas

- **Una librería multiplataforma y ya está**: si esa librería usa `unsafe` y
  bloquea el hilo, arrastra sus problemas a todo el proyecto. Se revisa en el
  *spike*.
- **Captura en el hilo de la interfaz**: congelaría la ventana con tráfico de
  gigabitros.