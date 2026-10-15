# Fixtures (capturas y vectores de prueba)

Datos que usa el proyecto para probarse. **Los crea la Persona A**, pero todos
los equipos los consumen y cualquiera puede añadir los suyos.

| Carpeta | Qué contiene | Quién los usa |
|---------|--------------|---------------|
| `minimos/` | Tramas mínimas escritas a mano: una por protocolo, y sus variantes truncadas y malformadas | B (pruebas del decodificador), C (datos de la interfaz) |
| `reales/` | Capturas pequeñas y **anonimizadas** de tráfico propio de quien las aporta | los tres, para pruebas de integración |
| `dorados/` | Trama → descripción esperada. Son la prueba de que un cambio en el decodificador es intencionado | B los escribe, A y C verifican que no cambian |
| `carga/` | Ficheros grandes para medir rendimiento y presión de memoria | A, para las cifras del Sprint 4 |

## Reglas de seguridad

1. Los `.pcap` están en `.gitignore`: **no se suben solos**.
2. Lo que sí se versiona son tramas mínimas escritas a mano (unos pocos bytes).
   Son material de pruebas, no tráfico real.
3. Para añadir una captura real, revisa antes [`../SECURITY.md`](../SECURITY.md):
   sin contraseñas, sin cabeceras de autenticación, sin nombres de personas ni
   de dominio internos, menos de 1 MB.
4. Nadie borra una fixture que usa otro. Si sobra algo, se avisa antes.
5. Nada de capturas de redes o equipos de terceros. Ni de pruebas ajenas.

## Convención de nombres

```text
minimos/  ethernet-ipv4-udp-basico.hex      # bytes en hexadecimal, con comentario
          ipv4-ihl-invalido.hex
reales/   2026-03-loopback-dns.pcap
dorados/  ethernet-ipv4-udp-basico.json     # salida esperada, decodificada
carga/    100k-paquetes.pcap
```

Cada fixture mínima lleva en un comentario qué caso cubre y por qué es
interesante. Una fixture sin explicar para qué sirve, no es una prueba: es
ruido.

---

## Guía de anonimización

Las capturas salen de tráfico propio, y aun así **no son datos tuyos para
publicar**. Un `.pcap` puede contener contraseñas, cabeceras de autenticación,
nombres de dominio internos, direcciones internas, nombres de usuario y metadatos
de dispositivos. Antes de subir una, hay que limpiarla.

### Regla de oro

**Ante la duda, no la subas.** Una captura vale menos que un fin de semana de
trabajo: siempre hay otra.

### Herramientas

Wireshark trae dos que hacen casi todo el trabajo:

| Herramienta | Qué hace |
|-------------|----------|
| `editcap` | Recorta, filtra y convierte ficheros. Viene con Wireshark |
| `mergecap` | Une varios ficheros en uno |
| `tcprewrite` | Reescribe direcciones MAC e IP. Viene con `tcprewrite` (paquete `tcprewrite-tools`), no con Wireshark |

Si no tienes nada de eso instalado, `editcap` está en la mayoría de paquetes de
Wireshark para Linux, macOS y Windows.

### Procedimiento

```bash
# 1. Guardar solo el tráfico que interesa (una conversación, un filtro...)
tshark -r captura-bruta.pcap -Y 'tcp.port==443' -w captura-filtrada.pcap

# 2. Sustituir direcciones MAC por unas inventadas
tcprewrite --infile=captura-filtrada.pcap \
           --dst-macaddr=02:00:00:00:00:01 --dst-macmap=0.00:00:00:00:00:01/02:00:00:00:00:01 \
           --outfile=captura-mac-clean.pcap

# 3. Revisar a ojo, en la propia herramienta, que no queda nada identificable
tshark -r captura-mac-clean.pcap -V | less
```

### Qué buscar antes de subir

- [ ] Nombres de dominio, sobre todo los internos (`*.corp`, `*.local`, nombres
      de servidor)
- [ ] Direcciones IP privadas identificables: `192.168.x.x` y `10.x.x.x` dan
      información sobre tu red
- [ ] Direcciones MAC de personas y de dispositivos (la MAC revela el
      fabricante y a veces el nombre del equipo)
- [ ] Cabeceras `Authorization`, `Cookie`, cabeceras `Proxy-` o `X-` propias
- [ ] Cadenas de texto en la carga útil: mensajes, nombres de fichero, rutas
- [ ] Nombres de usuario, correos, identificadores de sesión
- [ ] Marcas de tiempo: un `.pcap` dice exactamente cuándo estuviste en la red.
      Si hace falta, se desplazan todas las fechas con `tcprewrite --fixcrc`

### Dos atajos útiles

**Truncar la carga útil.** Si solo te interesa la estructura de las cabeceras,
`editcap` puede quedarse con los primeros bytes de cada paquete y evitar
problemas:

```bash
tshark -r captura.pcap -w solo-cabeceras.pcap -c 40
editcap -s 64 captura.pcap solo-64-bytes.pcap     # 64 bytes por paquete
```

**Ficheros de prueba sin datos de nadie.** Para probar el parser no hace falta
tráfico real: una trama de DNS escrita a mano tiene el mismo valor para un test
unitario y no tiene ningún riesgo. Usa `reales/` para lo que necesite realidad
(ordenar columnas, medir rendimiento, ver la interfaz con mil paquetes) y
`minimos/` para todo lo demás.

### Qué hacer si te equivocas

Si has subido una captura con datos que no deberían estar, **no la borres en
silencio**. Avisa al resto del equipo, dímelo al responsable del repositorio y
se sustituye. Borrar un fichero de git no borra el historial: el objeto sigue
en la historia y en las cachés de quien lo clonó.
