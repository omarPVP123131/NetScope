# Bocetos de la interfaz

> Bocetos en texto para que la Persona C no maquete sobre una idea imaginaria, y
> para que las tres vean lo mismo cuando hablen de «la lista» o «el detalle».
> No son maquetas: son la estructura de lo que tiene que existir.

---

## 1. La ventana principal

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ ▣ NetScope   [Abrir ▾] [● Capturar] [⏹]  ┌ Filtro ┐  [Exportar ▾]        │
├──────────────────────────────────────────┬─────────────────────────────────┤
│  # │ Hora      │ Origen        │ Destino │ DETALLE                          │
│    │           │               │         │ ┌────────────────────────────────┐ │
│  1 │ 10:23:41  │ 10.0.0.1      │ 10.0.0. │ │▾ Ethernet II            0-13  │ │
│    │  ⚠ 60 B   │ :52000        │ .2:443  │ │  Destino  ff:ff:ff:ff:ff:ff  │ │
│  2 │ 10:23:41  │ 10.0.0.2      │ 10.0.0. │ │  Origen    02:00:00:00:00:01  │ │
│    │  74 B     │ :443          │ .1:5200 │ │  EtherType 0x0800 (IPv4)      │ │
│  3 │ 10:23:42  │ 10.0.0.1      │ 10.0.0. │ ├────────────────────────────────┤ │
│    │  1514 B   │ :52000        │ .2:443  │ │▾ IPv4                   14-33 │ │
│  4 │ 10:23:42  │ 10.0.0.2      │ 10.0.0. │ │  Versión      4              │ │
│    │ ⚠ 96 B    │ :443          │ .1:5200 │ │  IHL         5 (20 bytes)     │ │
│  … │  …        │  …            │  …      │ │  Longitud    1500            │ │
│    │           │               │         │ │  TTL         64              │ │
│    │           │               │         │ │  Protocolo   6 (TCP)         │ │
│    │           │               │         │ │  Origen      10.0.0.1        │ │
│    │           │               │         │ │  Destino     10.0.0.2        │ │
│    │           │               │         │ ├────────────────────────────────┤ │
│    │           │               │         │ │▾ TCP                    34-53 │ │
│    │           │               │         │ │  Puerto origen  52000         │ │
│    │           │               │         │ │  Puerto destino 443           │ │
│    │           │               │         │ │  Banderas     SYN             │ │
│    │           │               │         │ └────────────────────────────────┘ │
│    │           │               │         ├─────────────────────────────────┤
│    │           │               │         │ BYTES                            │
│    │           │               │         │ 0000  02 00 00 00 00 01  ff ff  ff │ │
│    │           │               │         │       ff ff ff 08 00 45 00  [███]│ │
│    │           │               │         │ 0010  05 dc ab cd 00 00 40 06     │ │
│    │           │               │         │ 0020  9c 31 0a 00 00 01 0a 00 00 02│ │
├──────────────────────────────────────────┴─────────────────────────────────┤
│ 1.204 paquetes · 2,1 MB · 37 con errores · captura 0:42                        │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Las cuatro zonas y para qué son

| Zona | Contenido | Pregunta que responde |
|------|-----------|----------------------|
| **Lista** | Todos los paquetes, con columnas | «¿cuál de estos me interesa?» |
| **Detalle** | Árbol de capas y campos, con offset | «¿qué significa este paquete?» |
| **Bytes** | Volcado hexadecimal y ASCII | «¿dónde está eso en la trama?» |
| **Estado** | Totales, errores, progreso | «¿esto está funcionando bien?» |

Las tres primeras están **siempre a la vista y sincronizadas**: al elegir un
paquete en la lista, se actualizan las otras dos; al elegir un campo en el
detalle, se resalta su rango en los bytes. Ese es el bucle principal del
programa y todo lo demás es secundario.

---

## 3. Detalle de la lista

```text
  #  │ Hora      │ Origen           │ Destino          │ Protocolo │ Longitud │ Info
────┼───────────┼──────────────────┼──────────────────┼───────────┼──────────┼──────────
  1  │ 10:23:41. │ 10.0.0.1:52000   │ 10.0.0.2:443     │ TCP       │      74 B │ [SYN]
  2  │ 10:23:41. │ 10.0.0.2:443     │ 10.0.0.1:52000   │ TCP       │      74 B │ [SYN, ACK]
  3  │ 10:23:42. │ 10.0.0.1:52000   │ 10.0.0.2:443     │ TCP       │    1514 B │ [ACK]  ⚠
  4  │ 10:23:42. │ 10.0.0.2:443     │ 10.0.0.1:52000   │ TCP       │      96 B │ [ACK]   ⚠ truncado
```

**Los cuatro estados que hay que distinguir** (es el requisito de que «el
programa no se cae» sea visible, no un detalle de estilo):

| Marca | Significado |
|-------|-------------|
| — | Todo correcto |
| ⚠ | Algún campo no cuadra: checksum, longitud |
| ⚠✂ | Truncado: la captura no guardó el paquete entero |
| ⛔ | No se pudo interpretar el protocolo |

Además del color, cada estado lleva **icono y texto**, para no depender solo de
los colores.

---

## 4. Detalle: qué se ve de cada capa

```text
▾ Ethernet II                      0-13      (14 bytes)
    Destino        ff:ff:ff:ff:ff:ff    [0x00-0x05]   broadcast
    Origen         02:00:00:00:00:01    [0x06-0x0b]
    EtherType      0x0800               [0x0c-0x0d]   IPv4
▾ IPv4                             14-33     (20 bytes)
    Versión        4                   [0x0e]
    IHL            5 · 20 bytes        [0x0e]
    Longitud total 1500                [0x10-0x11]
    Identificación 0xabcd              [0x12-0x13]
    Banderas       Don't Fragment      [0x14]
    TTL            64                  [0x16]
    Protocolo      6 · TCP             [0x17]
    Checksum       0x1c46  ✓           [0x18-0x19]
    Origen         10.0.0.1            [0x1a-0x1d]
    Destino        10.0.0.2            [0x1e-0x21]
▾ TCP                              34-53     (20 bytes)
    Puerto origen  52000               [0x22-0x23]
    Puerto destino 443                 [0x24-0x25]
    Secuencia      0x00000001           [0x26-0x29]
    Acuse          0x00000000           [0x2a-0x2d]
    Offset datos   5 · 20 bytes        [0x2e]
    Banderas       SYN                 [0x2f]
    Ventana        64240               [0x30-0x31]
▸ Carga útil                       54-1473   (1420 bytes)   [click para ver los bytes]
```

Cuatro cosas que este boceto exige al modelo:

1. **El offset va en cada campo**, no solo en la capa. Es lo que convierte el
   programa en una herramienta y no en una tabla.
2. **El nombre del campo es estable** (`"Puerto origen"`, no `"puerto origen"`
   con mayúscula arbitraria). Se busca por nombre en toda la aplicación.
3. **Los campos opcionales se colapsan** (`▸ Carga útil`), para que un paquete
   con veinte capas siga siendo legible.
4. **El checksum correcto se marca con ✓**, y el incorrecto con ✗ y un motivo.
   No es un error: una captura hecha en un punto medio del enlace siempre da
   un checksum que no cuadra.

---

## 5. Vista de bytes

```text
0000  02 00 00 00 00 01 ff ff  ff ff ff ff 08 00  |..............|
0010  45 00 05 dc ab cd 40 00  40 06 1c 46 0a 00  |E....@. ...  |
0020  00 01 0a 00 00 02 [cb 90 00 00] 01 bb 01 00  |.............|
0030  00 00 50 02 ff ff [00 00 00 00] 00 00 00 00  |P............|
```

- Los corchetes marcan **los bytes del campo seleccionado** en el detalle.
- Al pasar el ratón por un byte, sale un tooltip con su offset y su valor.
- Botón de copiar: hexadecimal plano y hexadecimal con separación.
- Botón de «solo este paquete» y «solo este flujo».

---

## 6. Filtros

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ Filtro:  tcp && port 443                                             [ × ] │
│          842 de 1.204 paquetes                                          │
└────────────────────────────────────────────────────────────────────────────┘
```

- **El texto es el filtro.** Lo que escribe la persona es lo que se ejecuta.
- **El resultado se muestra siempre**: «842 de 1.204 paquetes», o
  «Ningún paquete coincide con el filtro» si no hay resultados.
- **Los errores son/amables**: `puerto 443x` →
  «No entiendo la parte `443x` después de `port`. ¿Querías decir `port 443`?».
- Historial con las flechas arriba y abajo.
- Filtros guardados con nombre, en una lista desplegable.

---

## 7. Estadísticas

```text
┌ Resumen ──────────────────────────────────────────────────────────────────┐
│ 1.204 paquetes · 2,1 MB · 4,8 s de captura · 37 con errores              │
│                                                                           │
│ Protocolo   paquetes      bytes   │  Conversación        bytes           │
│ TCP             842     1,92 MB  │  10.0.0.1:52000 → 10.0.0.2:443  1,8M │
│ UDP             210   442,3 kB   │  10.0.0.2:53 → 10.0.0.1:5353   18 kB │
│ ARP              96     9,4 kB   │  ...                                 │
│ DNS              34     7,1 kB   │                                      │
│ ICMP             22     2,0 kB   │                                      │
│ Desconocido      ...        ...   │                                      │
└───────────────────────────────────────────────────────────────────────────┘
```

El número que más importa es **«37 con errores»**. Es la medida de si un
fichero es limpio, y va siempre visible.

---

## 8. Captura en vivo

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ ▣ NetScope   [Abrir ▾] [● Capturar] [⏹ Parar]  ┌ Filtro ┐  [Exportar ▾] │
│                  interfaz: eth0  ·  capturando 00:42  ·  3.218 paquetes/s │
└────────────────────────────────────────────────────────────────────────────┘
```

Cuando el permiso falta:

```text
┌────────────────────────────────────────────────────────────────────────────┐
│  No se puede capturar en esta interfaz                                     │
│                                                                            │
│  Capturar tráfico requiere privilegios de administrador en Linux.          │
│                                                                            │
│  Tienes dos opciones:                                                      │
│    · instalar el componente de captura:  netscope-instalar                │
│    · abrir un fichero .pcap sin privilegios                                │
│                                                                            │
│                                            [Instalar]  [Abrir fichero]     │
└────────────────────────────────────────────────────────────────────────────┘
```

Este mensaje es el resultado de la decisión del ADR-0007: **explica el
problema, da las dos salidas y no deja al usuario encerrado en un mensaje de
error.**

---

## 9. Lo que estos bocetos exigen a los otros

| De C (interfaz) | De B (protocolos) | De A (captura) |
|---|---|---|
| El offset por campo | Cada campo con su rango | Nada |
| El nombre estable del campo | Nombres en una convención común | Nada |
| El estado por paquete | Los tres niveles de error | Que los truncados se marquen |
| El colapso de payload | Saber dónde acaba cada capa | Nada |
| Filtros en la caja de búsqueda | Protocolos cerrados, sin números sueltos | El mismo lenguaje de filtro |
| El contador de errores | Errores por paquete | Nada |
| Paquetes/s en captura | Nada | El pipeline las calcula |

**Este documento es la prueba de que el modelo de B y las necesidades de C
encajan.** Si algo de la columna de B no existe todavía en el contrato, es un
hueco del Sprint 1, no de la semana 7.