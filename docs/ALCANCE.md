# Alcance de NetScope

> **14 de octubre de 2026.** Este documento define qué es NetScope y, sobre todo,
> **qué no es**. Sin esta página, un proyecto con tres personas y seis meses
> crece hasta romperse.

---

## 1. Qué significa «comparable a Wireshark»

La decisión es: **el objetivo es tener las mismas *categorías* de función que
Wireshark, sobre un conjunto de protocolos mucho más pequeño.**

No la misma cobertura. Wireshark entiende más de dos mil protocolos porque son
veinte años de trabajo y ciento cincuenta personas. NetScope, en doce sprints, no
llega ni de lejos. Lo que se busca es esto:

| Función | En Wireshark | En NetScope |
|--------|-------------|-------------|
| Abrir un fichero de captura | ✅ | ✅ PCAP y PCAPNG |
| Ver la lista de paquetes | ✅ | ✅ con columnas configurables |
| Ver el detalle campo a campo | ✅ | ✅ con offset y valor |
| Ver los bytes en hexadecimal | ✅ | ✅ con resaltado sincronizado |
| Filtrar por protocolo, dirección y puerto | ✅ | ✅ con lenguaje propio |
| Seguir un flujo TCP | ✅ | ✅ con reensamblado |
| Seguir una conversación (netflow) | ✅ | ✅ |
| Capturar en vivo | ✅ | ✅ en los tres sistemas |
| Estadísticas (protocolos, conversaciones, E/S) | ✅ | ✅ básicas |
| Exportar a fichero | ✅ | ✅ `.pcap`, CSV y JSON |

**Igualdad en las categorías, no en la profundidad.** Cuando Wireshark resuelve
un nombre con punteros comprimidos y NetScope muestra los bytes, eso es
aceptable: es un proyecto educativo con seis meses de duración.

**La prueba de que se ha llegado:** una persona abre un `.pcap` de tráfico real,
encuentra lo que busca filtrando, sigue una conexión TCP de principio a fin y
exporta lo que le interesa. **Sin una sola función que necesite leer el código.**

---

## 2. Qué entra

### Los protocolos

| Protocolo | Por qué | Prioridad |
|-----------|---------|-----------|
| Ethernet II | Base de todo | obligatorio |
| VLAN / QinQ | Aparece en cualquier red gestionada | obligatorio |
| ARP | Constantemente en las redes locales | obligatorio |
| IPv4 | El 80 % del tráfico | obligatorio |
| IPv6 | Creciente, y sus cabeceras de extensión son el ejercicio difícil | obligatorio |
| TCP | El protocolo interesante | obligatorio |
| UDP | Transporte de DNS, DHCP y capturas de herramientas | obligatorio |
| ICMPv4 y ICMPv6 | Para el `ping` y para entender los errores de red | obligatorio |
| DNS | Cada red tiene nombres, y su formato es una buena lección de punteros | obligatorio |
| DHCP | Aparece en cada arranque de equipo | obligatorio |
| HTTP | Es lo que la gente realmente busca en un análisis | deseable |
| TLS (solo *ClientHello* y *ServerHello*) | Revela el protocolo negociado sin descifrar nada | deseable |

**"Deseable" significa que se hace si sobra tiempo, no que se prometa.** Nadie
debe feel culpable en el Sprint 12 porque no haya TLS.

### Las funciones

**Esenciales** (sin ellas no hay producto)

1. Abrir ficheros PCAP y PCAPNG, incluidos los dañados.
2. Lista de paquetes con columnas, ordenación y selección múltiple.
3. Detalle campo a campo con offset, valor y notas.
4. Vista de bytes con resaltado del campo seleccionado.
5. Filtros por protocolo, dirección IP, puerto y texto libre.
6. Seguir flujos TCP, UDP y DNS.
7. Exportar la selección a `.pcap` y a CSV.
8. Estadísticas: totales, por protocolo, por conversación y caudal.

**Importantes** (marcan la diferencia)

9. Captura en vivo con inicio y parada, sin congelar la interfaz.
10. Estadísticas en vivo mientras se captura.
11. Búsqueda incremental dentro de la captura.
12. Generador de paquetes de laboratorio.
13. Filtros guardados y atajos de teclado.
14. Modo claro y modo oscuro.

**Estiramiento** (si el proyecto va sobrado)

15. TLS, HTTP.
16. Exportación a JSON para análisis posteriores.
17. Tema claro/oscuro configurable más allá del interruptor.
18. Comparar dos capturas.

---

## 3. Qué no entra, y por qué

Esta lista es la que más tiempo ahorra si está escrita antes de empezar.

| No entra | Motivo |
|----------|--------|
| **TLS, IPsec, SSH, FTP y demás** | Descifrar no es el objetivo. Se muestra el *ClientHello*, que ya cuenta a quién se habla. |
| **Filtros BPF compilados** | Es un lenguaje en sí mismo, con su propio parser y sus propios números. El filtro propio cubre el caso de uso. |
| **Inyección y envío de paquetes** | Fuera del proyecto por decisión. El generador hace tráfico local y controlado. |
| **Escaneo y descubrimiento de red** | Lo mismo. |
| **Más de 12-15 protocolos** | Añadir el protocolo número trece cuando el número cuatro está a medias es cómo mueren estos proyectos. |
| **Escritura de ficheros PCAPNG** | Se escribe PCAP clásico, que es mucho más simple y Wireshark lo lee igual. |
| **Multiusuario y sincronización** | Es una aplicación de escritorio. |
| **Interfaz web o móvil** | Otra aplicación, otro lenguaje. |
| **Compatibilidad conestadísticas de Wireshark** | Los formatos internos no son públicos. Se es compatible con los ficheros, no con el software. |
| **Un motor de plugins** | Multiplicaría el trabajo por diez. Añadir un protocolo va por el registro. |

Si algo de esta lista entra, **no se quita nada de ella**: entra en el Sprint 13
o no entra. Acortar una lista definida es más fácil que mantener dos informes.

---

## 4. Qué significa «terminado»

Para que el proyecto no termine por cansancio y no por decisión.

### v0.1 — «se puede usar» (Sprint 12, abril de 2027)

- Abre ficheros PCAP y PCAPNG, incluidos los truncados.
- Decodifica los protocolos obligatorios.
- Lista, detalle y bytes.
- Filtros y búsqueda.
- Estadísticas.
- Exportación a `.pcap` y CSV.
- Captura en vivo en los tres sistemas.
- Binarios portátiles en la sección de releases.

### v0.5 — «está pulido» (siguiente, sin fecha)

- Todo lo de v0.1, más lo que no entra: HTTP, TLS, comparación de capturas,
  temas.

### Qué NO es el criterio de éxito

- Número de protocolos soportados.
- Número de líneas de código.
- Parecerse a Wireshark por fuera.

**El criterio es uno:** que alguien que no ha escrito el código use NetScope
para algo real y entienda lo que ve. Si eso pasa con ocho protocolos, el
proyecto ha funcionado. Si no pasa con quince, no ha funcionado.

---

## 5. Cómo se protege el alcance

Cuatro reglas, sin excepciones:

1. **Lo de "deseable" no entra hasta que todo lo esencial esté en verde.** No
   antes, ni «porque es rápido», ni «porque ya está a medias».
2. **Un sprint, tres entregables.** Añadir un cuarto significa quitar otro.
3. **Lo que va a medias se termina o se tira.** Un Ethernet a medio hacer no
   cuenta.
4. **El Sprint 5 y el 11 bajan alcance de antemano** por las fiestas. Está en el
   calendario, no se improvisa en diciembre.

Cuando algo se caiga de la lista, **se escribe aquí**. Una lista que se negocia
en silencio durante seis meses acaba pareciendo un plan que nunca existió.

---

## 6. El riesgo honesto

El reensamblado TCP es la función que más veces ha hecho fracasar proyectos de
este tamaño: reordenamiento, retransmisiones, secuencia que desborda, capturas
que empiezan a mitad de una conexión. No es un `if`, es un problema de estado.

Está aislado en los sprints 8 y 9 **a propósito**: si se retrasa, se cae el
resto y se mantiene lo demás. Un NetScope sin flujos es un analizador de
cabeceras muy bueno; un NetScope a medias de entrega en mayo no es nada.