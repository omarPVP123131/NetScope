# ADR-0003 · PCAP primero, PCAPNG después

**Estado:** propuesta · **Fecha:** Sprint 1 · **Personas afectadas:** A

## Contexto

Hay dos formatos de captura que importan: **PCAP** (clásico, con 24 bytes de
cabecera global) y **PCAPNG** (por bloques, con secciones e interfaces
potencialmente distintas). PCAPNG es el formato que genera Wireshark por defecto
desde hace años, así que sooner o later habrá que leerlo.

Los dos son **entradas no confiables**: un fichero puede estar truncado a mitad,
tener campos de longitud absurdos o estar generado por una herramienta con
extensiones extrañas.

## Decisión

1. El **Sprint 2** implementa la lectura de PCAP clásico, validando cabecera y
   registros antes de usarlos.
2. El **Sprint 3** añade PCAPNG, reutilizando la misma validación.
3. La lectura de ficheros se hace con código propio salvo que en el *spike* del
   Sprint 2 la biblioteca candidata demostrarse equivalente y más segura
   (ver [ADR-0006](0006-lectura-de-pcap.md)).
4. Ningún fichero puede hacer que la aplicación termine de forma anormal. Un
   error de cabecera es un error visible; una trama corrupta es un aviso en la
   lista.

## Consecuencias

**A favor**

- El camino crítico (abrir un `.pcap`) se conoce y se controla entero, que es
  justo lo que se quiere aprender en el proyecto.
- Sin dependencias del sistema para la mitad del producto: la instalación es
  trivial hasta el Sprint 6.
- El parser se puede probar con ficheros de un byte, de uno vacío y de uno con
  el número mágico cambiado.

**En contra**

- Reimplementar un formato binario es trabajo extra frente a usar una
  biblioteca. Se acepta: es parte del objetivo educativo.
- Los ficheros de producción con extensiones exóticas (o PCAP de 64 bits) precisam
  una revisión en el Sprint 3.

## Alternativas descartadas

- **Solo usar la biblioteca del sistema**: crea una dependencia del sistema
  operativo en el producto completo y esconde justo la parte que más conviene
  entender.
- **PCAPNG primero**: es más complejo (secciones, interfaces, options) y no
  aporta nada al primer objetivo del proyecto.