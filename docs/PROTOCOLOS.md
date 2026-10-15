# Notas de estudio de protocolos

Este documento es la libreta de trabajo de la **Persona B** (protocolos). Antes
de escribir un decodificador, hay que tener estas respuestas. No hace falta
apuntarlas todas en el documento: basta con no descubrirlas en mitad de la
implementación.

---

## Regla general para cualquier protocolo

Antes de escribir código, responde a estas seis preguntas:

1. **¿Cuál es la longitud fija de la cabecera?** Si es variable, ¿de qué campo
   sale y cuál es su valor mínimo y máximo razonable?
2. **¿Qué endianness?** ¿Hay campos de bits que cruzan los límites de byte?
3. **¿Qué campos son de longitud variable y dónde empieza el payload?**
4. **¿Qué campos pueden ser absurdos?** (longitudes, offsets, contadores)
5. **¿Qué identificador decide a qué protocolo corresponde?** (EtherType, número
   de protocolo IP, puerto)
6. **¿Qué pasa con el paquete truncado?** ¿Qué campos se pueden mostrar
   igualmente?

---

## Ethernet II

- Longitud fija: 14 bytes (6 destino + 6 origen + 2 EtherType).
- Añadidos: 802.1Q VLAN (4 bytes por etiqueta, encadenables), QinQ, y la
  etiqueta de stack que usan algunos switches.
- El destino puede ser broadcast (`ff:ff:ff:ff:ff:ff`), multicast con el bit
  menosSignificant a 1, o unicast.
- Una trama Ethernet normal mide entre 64 y 1518 bytes (sin contar el FCS).
- **Pendiente**: recorrido de etiquetas VLAN (Sprint 2).

## ARP

- 28 bytes para Ethernet/IPv4. Campos `htype`, `ptype`, `hlen`, `plen` indican
  los tamaños de las direcciones, así que **no siempre son 6 y 4**.
- Los dos primeros bytes del campo de operación: 1 = solicitud, 2 = respuesta.
  Los valores 3 y 4 son respuestas inversas (ARP inverso / InARP), poco frecuentes.
- En una solicitud, la MAC de destino suele estar a cero.
- **Trampa**: en una solicitud, el campo "dirección hardware de destino" está a
  cero; no es un paquete corrupto.

## IPv4 (RFC 791)

- Longitud variable por el campo **IHL** (en palabras de 32 bits, mínimo 5).
- El campo *longitud total* incluye la cabecera. Si es menor que la cabecera, es
  un paquete imposible.
- Bandera *Don't Fragment* y desplazamiento de fragmento comparten 16 bits: 3
  banderas y 13 bits de desplazamiento, expresados en múltiplos de 8 bytes.
- **Checksum**: solo sobre la cabecera. Un paquete con la suma de verificación
  incorrecta puede venir de una captura en un punto medio del enlace, no
  necesariamente estar corrupto.
- Opciones: pueden ser de variable longitud, incluso en paquetes antiguos.
- **Trampas**: `total_length` mayor que los bytes disponibles (captura con
  `snaplen` corto), `IHL` menor de 5, `IHL` mayor que `total_length`, y
  fragmentos que no son el primero (no tienen cabecera de transporte).

## ICMPv4

- 8 bytes de cabecera: tipo, código, checksum. Los tipos 0 y 8 son
  petición/respuesta de eco, y son los que verás en un `ping`.
- El resto de la carga depende del tipo. En un eco, los primeros 8 bytes son
  identificador y número de secuencia.
- El campo "checksum" cubre todo el mensaje ICMP.

## IPv6 (RFC 8200)

- Cabecera de tamaño **fijo** de 40 bytes. No hay IHL, no hay longitud de
  cabecera, no hay checksum.
- La longitud de carga útil es un entero de 16 bits, así que un datagrama
  IPv6 no pasa de 64 KB salvo que usen jumbograms.
- **Cadena de cabeceras de extensión**: el campo "siguiente cabecera" puede
  apuntar a Hop-by-Hop, Routing, Fragment, AH, ESP o Destination Options, cada
  una con su propio formato de longitud. Hay que recorrerlas para llegar al
  protocolo real.
- **Trampa**: los tres primeros bits de la versión se comprueban antes de nada;
  un valor distinto de 6 significa que no es IPv6, aunque el EtherType diga que
  lo es.

## TCP (RFC 9293)

- Longitud variable por el *data offset* (4 bits, en palabras de 32 bits).
- El byte de banderas va del bit 7 al 0: CWR, ECE, URG, ACK, PSH, RST, SYN, FIN.
  Los tres primeros no se usan en la cabecera fija.
- **Checksum**: obligatorio, y cubre una **pseudo-cabecera** con las direcciones
  IP, el número de protocolo y la longitud del segmento. Por eso, para
  verificarlo, el decodificador necesita conocer las IPs que trae la capa IP.
- No hay longitud total en la cabecera: la longitud la determina la capa IP. El
  `data offset` dice dónde acaba la cabecera y empieza la carga útil.
- Opciones de alineación a 4 bytes con semántica propia.
- **Trampas**: `data offset` menor de 5, segmentos sin PSH ni FIN de verdad,
  retransmisiones (mismo número de secuencia dos veces).

## UDP (RFC 768)

- 8 bytes fijos. El campo de longitud es el único sitio donde aparece el tamaño
  del datagrama.
- En **IPv4** el checksum puede ser 0 (no se calcula). En **IPv6** es
  **obligatorio** y 0 es un paquete inválido.
- El checksum incluye la misma pseudo-cabecera que TCP.

## DNS

- Cabecera de 12 bytes: identificador, banderas, cuatro contadores.
- Las banderas empaquetan QR, Opcode, AA, TC, RD, RA y RCODE en 16 bits:
  desempacar un byte en cuatro banderas es buen ejercicio.
- El protocolo se detecta por **puerto** (53), no por EtherType. Es el primer
  caso donde la decodificación depende del transporte, no de la capa de red.
- Los nombres usan compresión con punteros a offsets anteriores: es un caso
  clásico de índice que puede apuntar hacia atrás y provocar bucles si no se
  valida.

---

## Lista de comprobación de un decodificador nuevo

- [ ] Leer la especificación, no un tutorial.
- [ ] Escribir primero los **casos límite** en las pruebas: 0 bytes, 1 byte,
      longitud mínima, longitud imposible.
- [ ] Una prueba por cada rama de la lógica de campos de longitud.
- [ ] Una prueba con el checksum bien puesto y otra con el checksum roto.
- [ ] Una prueba con un campo de longitud mayor que los datos disponibles.
- [ ] Un test del corpus hostil (tramas mutadas a mano): no debe haber pánico.
- [ ] Registrar el protocolo en el registro, en un solo sitio.
- [ ] Actualizar el estado del protocolo en el README.

---

## Glosario de términos

| Término | Significado |
|---------|-------------|
| EtherType | Identificador de protocolo en una trama Ethernet |
| Siguiente cabecera | En IPv6, puntero al protocolo o a la siguiente extensión |
| Pseudo-cabecera | Bloque que se antepone al checksum de TCP y UDP, sin transmitir |
| `snaplen` | Número máximo de bytes guardados por paquete en una captura |
| Fragmento | Parte de un datagrama IP que no contiene la cabecera completa |
| Decodificador | Función que convierte bytes en campos con nombre y offset |
| Registro | Tabla que asocia un identificador de protocolo con su decodificador |