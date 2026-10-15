# Seguridad y uso responsable

NetScope se desarrolla para **aprendizaje, investigación y diagnóstico
autorizado**. Esta página explica qué se espera de quien lo usa y qué se espera
del proyecto.

---

## Uso autorizado

- Captura en vivo **únicamente** en redes y dispositivos sobre los que tengas
  autorización explícita. En muchas jurisdicciones, capturar el tráfico de
  terceros sin consentimiento es delito.
- En la mayoría de sistemas, capturar tráfico exige privilegios de administrador.
  Eso es una señal de alarma de seguridad, no un obstáculo que sortear.
- El **generador de paquetes** existe para montar redes de laboratorio
  controladas y para comprobar el propio analizador. No se usa para inyectar
  tráfico en redes reales.
- **No hay escaneo, no hay sniffing activo, no hay evasión de detección y no hay
  función de envío.** Queda fuera de alcance por decisión del proyecto, no por
  falta de tiempo.

## Datos sensibles

Un fichero `.pcap` puede contener:

- contraseñas y cabeceras de autenticación en texto claro;
- nombres de usuarios, correos y nombres de dominio internos;
- direcciones IP privadas y nombres de host;
- metadatos de dispositivos y usuarios.

Por eso:

- los `.pcap` están en `.gitignore` y no se suben automáticamente;
- las fixtures que se versionan son tramas mínimas escritas a mano, no tráfico
  real;
- quien añada una captura real la revisa antes: sin contraseñas, sin datos
  personales, menos de 1 MB;
- los ficheros de captura se tratan como **datos no confiables** al leerlos: el
  parser valida longitudes y límites antes de acceder a la memoria.

## Furto de datos en el parser

Un analizador de paquetes es, por naturaleza, un programa que lee ficheros que
otro ha escrito. Eso lo convierte en una superficie de ataque clásica:

| Ataque | Defensa prevista |
|--------|------------------|
| Longitud declarada enorme → reserva de memoria o bucle infinito | Ningún campo de longitud se usa para reservar sin límite superior |
| Offset o puntero fuera del buffer | Lecturas siempre a través de un lector con rango comprobado |
| Cabecera de longitud variable manipulada (IHL, data offset, VLAN) | Validación de longitud mínima y máxima antes de usarla |
| Ciclo de decodificación (un protocolo que se apunta a sí mismo) | Tope de capas por paquete y conjunto de visitados |
| Desbordamiento de entero al calcular longitudes | Operaciones con verificación explícita y pruebas al límite |
| Pánico por índice fuera de rango | Sin `unsafe`, sin indexado directo, con errores en lugar de pánico |

Cualquier fallo de seguridad en el parser se reporta de forma confidencial: no
abras una incidencia pública con el fichero ni con los pasos para reproducirlo.
Envíalo por privado a los responsables del repositorio.

## Privacidad de los usuarios

El proyecto no recopila telemetría, no envía datos a ningún sitio y no incluye
analítica de terceros. La captura en vivo es un proceso local y el usuario
puede detenerlo en cualquier momento.