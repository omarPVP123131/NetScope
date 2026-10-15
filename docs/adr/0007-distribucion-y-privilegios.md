# ADR-0007 · Distribución y privilegios para la captura

**Estado:** **aceptada** · **Decidida:** 14 de octubre de 2026 ·
**Personas afectadas:** A

---

## Contexto

Hay dos decisiones ya tomadas que **se contradicen**:

1. **Binarios portátiles**: se descarga, se descomprime y se ejecuta, sin
   instalar nada.
2. **Instalador que concede los privilegios** de captura y los retira al
   desinstalar.

Y ambas cosas hacen falta: la captura en vivo exige privilegios de
administrador en cualquier sistema, y ningún binario portátil puede
conseguirlos.

### Lo que exige cada sistema

| Sistema | Se puede capturar sin privilegios | Cómo se concede el permiso | Sistema operativo |
|---------|-----------------------------------|----------------------------|------------------|
| **Linux** | No | `sudo`, el bit `setuid` o *capabilities* (`CAP_NET_RAW`) en el binario | **Sí** |
| **Windows** | No | Npcap, que se instala aparte y es de una empresa externa | **Sí** |
| macOS | No | BPF, que exige que el binario esté firmado y con los privilegios adecuados | **Fuera de alcance** |

En Windows hay una limitación que no es nuestra: **Npcap lo instala una empresa
tercera**, con su propio instalador, y su licencia de distribución tiene
condiciones. No podemos redistribuirlo. Solo podemos detectarlo, explicarlo y
enviar al usuario a su instalador.

### El problema de Windows que nadie dice en voz alta

Un ejecutable sin **firma digital** hace que Windows muestre «Windows protegió
tu PC» y lo bloquee hasta que el usuario pulse «Ejecutar de todas formas». En un
proyecto de tres personas, esa es la primera impresión que se lleva alguien que
se descarga NetScope por Curiosity. Un certificado de firma cuesta entre 70 y
300 € al año.

## Decisión

### 1. Dos artefactos, no uno

| Artefacto | Para qué | Cuándo |
|-----------|----------|--------|
| **Binario portátil** (un ejecutable por sistema) | Leer ficheros, sin permisos, sin instalar nada | Siempre disponible en *releases* |
| **Instalador por sistema** | Instalar el programa y conceder el permiso de captura | Opcional, solo si se quiere capturar en vivo |

El binario portátil **arranca y avisa**: «puede abrir ficheros; para capturar en
vivo necesita instalar el componente de captura». Nunca falla en silencio.

### 2. Linux: *capabilities*, no `setuid` ni `sudo`

El instalador aplica `CAP_NET_RAW` al binario:

```bash
sudo setcap cap_net_raw,cap_net_admin+eip /opt/netscope/netscope
```

- **No es `setuid`**: un binario `setuid root` que se lleva un exploit es un
  problema serio. Una *capability* concreta limita mucho más el daño.
- **No es `sudo`**: pedir la contraseña del administrador cada vez que se abre
  el programa es una experiencia horrible y un ticked de seguridad enorme.

El instalador ofrece **desinstalar los permisos** al quitar el programa.

### 3. Windows: detectar, explicar y acompañar

El instalador comprueba si Npcap está presente. Si no lo está, explica qué es y
abre la página oficial. No lo instala por su cuenta ni lo redistribuye.

### 4. macOS: fuera de alcance, documentado como no soportado

El programa abre ficheros en macOS. La captura en vivo no está soportada y la
interfaz lo dice con un mensaje claro, no con un fallo.

### 5. Sin firma digital en la v0.1

Se asume el aviso de Windows. El README lo explica con capturas de pantalla
para que nadie piense que el programa es un virus.

## Consecuencias

**A favor**

- Quien solo quiere mirar una captura **no instala nada**. Es la mitad de los
  usuarios y la mitad del fricco eliminado.
- Los privilegios están contenidos en una *capability*, no en root.
- Se puede desinstalar dejando el sistema igual que estaba, que es lo que
  corresponde hacer.

**En contra**

- Dos artefactos que mantener en vez de uno.
- El instalador es trabajo de verdad: hay que probarlo en los tres sistemas y
  en máquinas limpias, y no se puede automatizar entero.
- El aviso de Windows sin firmar es una Conversación incómoda con quien se lo
  descarga. Está documentado, pero documentado no es lo mismo que resuelto.
- Los permisos de captura son, por definición, una superficie de ataque: el
  binario con `CAP_NET_RAW` es código de nivel privilegiado.

## Qué se hace en cada sprint

| Sprint | Trabajo |
|--------|---------|
| 10 | Backend de Linux, mensajes de permiso claros |
| 11 | Instalador de Linux, detección de Npcap en Windows, empaquetado |
| 12 | Binarios portátiles, página de instalación en el README con capturas de los avisos del sistema |

## Alternativas descartadas

- **Solo instalador**: obliga a instalar para leer una captura. Mal equilibrio.
- **Solo binario portátil con `sudo`**: el usuario teclea su contraseña de
  administrador cada vez. Y nadie lo hace dos veces.
- **Solo `setuid`**: funciona, pero es la peor de las tres opciones de seguridad.
- **Firmar los binarios**: es lo correcto y requiere presupuesto. Se anota
  como Sprint 13 con un presupuesto de 100-200 € al año.
- **macOS**: exige firmar, entitlement y un instalador propio. Tres cosas
  nuevas para el sistema menos usado por el equipo.