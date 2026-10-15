# Validación cruzada con Wireshark

> **Decisión del 14 de octubre de 2026.** NetScope interpreta protocolos; también
> lo hace Wireshark, que lleva veinte años haciéndolo. Cada vez que los dos no
> coinciden, uno de los dos se equivoca, y lo que no se comprueba es un error
> que se propaga hasta que alguien lo encuentra.

---

## La idea

Cada protocolo se interpreta de dos maneras independientes:

1. **NetScope**, con sus pruebas y su corpus de tramas.
2. **Wireshark**, con `tshark -T fields`, que extrae los campos ya
   desensamblados.

Si un campo dice `TTL 64` en NetScope y `64` en `tshark`, ambas cosas son
certainas. Si NetScope dice 64 y `tshark` dice 6, **alguien tiene un error** y
todavía no sabemos quién.

El objetivo **no** es que NetScope copie a Wireshark. Es que cada decisión de
interpretación quede contrastada con alguien que lleva dos décadas
haciéndolo, y que la comparación sea automática y repetible.

---

## Cómo funciona

`tshark` puede extraer campos concretos de un fichero:

```bash
tshark -r captura.pcap \
       -T fields \
       -e frame.number -e ip.src -e ip.dst -e ip.ttl \
       -e tcp.srcport -e tcp.dstport -e tcp.flags.syn \
       -E separator=, -E header=y -E quote=n
```

Sale una tabla con los valores ya interpretados:

```text
frame.number,ip.src,ip.dst,ip.ttl,tcp.srcport,tcp.dstport,tcp.flags.syn
1,10.0.0.1,10.0.0.2,64,52000,443,1
2,10.0.0.2,10.0.0.1,64,443,52000,0
```

La operación de NetScope tiene que producir lo mismo para las mismas tramas.
La comparación de ambos ficheros, paquete a paquete, es lo que se versiona en
`fixtures/dorados/`.

---

## Qué se compara y qué no

| Se compara | No se compara |
|------------|---------------|
| Direcciones IP y MAC | Los nombres que Wireshark resuelve con DNSSEC o bases de datos |
| Puertos | La etiqueta de protocolo cuando hay puertos ambiguos |
| TTL, longitudes, número de secuencia | Tiempos relativos calculados por Wireshark |
| Banderas de TCP, una a una | El significado de las banderas reservadas |
| Códigos de operación y códigos de error | Las colas de trabajo internas |
| Identificadores de petición DNS | El campo `Name` con nombres largos |
| El número de capa que NetScope asigna | El orden de presentación |

**Regla:** solo se comparan valores que tienen una correspondencia clara y
estable. Cuando no la tienen, la comprobación es manual y se anota como tal.

---

## Cómo se ejecuta

### A mano, al terminar un decodificador

```bash
# 1. NetScope vuelca sus tramas al mismo formato
netscope volcar capturas/fixtures/minimos/ipv4-basico.pcap

# 2. tshark extrae los mismos campos
tshark -r capturas/fixtures/minimos/ipv4-basico.pcap -T fields \
       -e frame.number -e ip.src -e ip.dst -e ip.ttl

# 3. Se comparan a ojo las dos salidas
diff <(netscope volcar ...) <(tshark -r ...)
```

### En integración continua

El job compara el volcado de NetScope con el de `tshark` sobre todo el corpus
de `fixtures/`. Si divergen, **el CI falla**.

```yaml
- name: Instala Wireshark
  run: sudo apt-get install -y tshark
- name: Comparar con Wireshark
  run: ./herramientas/comparar-con-wireshark.sh fixtures/
```

Una diferencia **es** un fallo del CI. Si resulta que la diferencia es
legítima (un campo que solo NetScope interpreta), se anota en el fichero de
excepciones con un motivo escrito. Nunca se relaja la comparación: se documenta
por qué ese caso es distinto.

---

## Los dos riesgos, y cómo se cubren

### ⚠️ Que NetScope copie un error de Wireshark

Es el riesgo serio. Si comparamos sin pensar, acabamos reproduciendo el
comportamiento de otra herramienta, incluida cualquier rareza histórica.

**Cómo se cubre:**

- Las pruebas de NetScope se escriben **contra la especificación** (RFC), no
  contra `tshark`. `tshark` es el segundo filtro, no la fuente.
- Cuando NetScope y Wireshark discrepen, lo primero es leer la RFC. Si la RFC
  dice que NetScope tiene razón, **se anota la discrepancia y se sigue con
  NetScope**. Wireshark tiene veinte años de compatibilidad con el mundo real
  y a veces la especificación no dice lo que pasó.
- El caso de TLS de Wireshark con escrituras mal alineadas es exactamente el
  ejemplo: la especificación dice una cosa, la realidad otra.

### ⚠️ Que Wireshark tenga una Decisión distinta y razonable

También pasa. `tshark` puede llamar «puerto bajo 1024» a algo que NetScope llama
por su número de servicio. En ese caso la comparación no aplica y se documenta.

**Regla:** una discrepancia se resuelve leyendo la especificación, no votando.

---

## Qué hay que tener instalado

```bash
# Linux
sudo apt-get install tshark
# macOS (en macOS se compila, pero tshark funciona sin más)
brew install wireshark
# Windows: el instalador de Wireshark incluye tshark
```

En CI, `sudo` no hace falta porque `tshark` lee ficheros sin tocar la red.

---

## Cuándo se hace

| Momento | Comprobación |
|---------|-------------|
| Al terminar cada decodificador | Manual, sobre las tramas mínimas de `fixtures/minimos/` |
| Al final de cada sprint | El corpus completo, en local |
| En cada PR que toque `netscope-protocol` | Automática, en CI |
| Al publicar la v0.1 | Sobre las capturas reales de `fixtures/reales/` |

---

## El resultado de esta práctica

Lo mejor que aporta no es encontrar errores: es que **la lista de diferencias
entre NetScope y Wireshark está escrita y es revisable**. Esa lista es, a la
vez, el informe más honesto que el proyecto puede hacer sobre sí mismo: dice
exactamente en qué se parece a la herramienta de referencia y en qué no.