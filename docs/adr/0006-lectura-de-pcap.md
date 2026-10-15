# ADR-0006 · Lectura de PCAP: código propio o biblioteca

**Estado:** **abierta** · **Se decide en:** Sprint 2, primer día · **Personas afectadas:** A y B

## Contexto

Para leer ficheros `.pcap` hay, como mínimo, tres caminos:

1. **Código propio** sobre `std::io`, sin dependencias.
2. **`pcap-parser`**, una biblioteca de Rust puro centrada en PCAP.
3. **`pcap`**, el enlace con libpcap del sistema.

El criterio del proyecto es doble: aprender a leer formatos binarios **y**
mantener el producto instalable en cualquier máquina sin instalar nada.

## Preguntas que debe responder el *spike* (medio día)

- [ ] ¿`pcap-parser` cubre PCAPNG o solo PCAP clásico?
- [ ] ¿Maneja los cuatro números mágicos (dos endiannesses × dos
      resoluciones de marca de tiempo)?
- [ ] ¿Valida longitudes o se fía del fichero? Si se fía, ¿cuánto código hay que
      añadir por nuestra cuenta para no leer fuera de rango?
- [ ] ¿Tiene `unsafe`? ¿Cuánto?
- [ ] ¿Cómo generar ficheros de prueba para cada variante (32/64 bits, nanos,
      little/big endian) si usamos una biblioteca: ¿escribimos un generador de
      PCAP nosotros o usamos una herramienta externa?

## Estado actual

**Ninguna decisión tomada.** Este documento existe para que la decisión se tome
con criterios y por escrito, no por inercia.

## Recomendación provisional

Empezar con **código propio** (es la parte formativa del proyecto) y se evalúa la
biblioteca en paralelo. Si tras el *spike* la biblioteca cubre PCAP, PCAPNG y
validación, se adopta para lectura y se conserva el código propio como
referencia y como generador de fixtures. La decisión se toma en el primer día
del Sprint 2 y se anota aquí.

## Consecuencias de cada salida

| Opción | Ganancia | Coste |
|--------|----------|-------|
| Código propio | Control total, sin dependencias, formativo | ~300-400 líneas y un par de días de pruebas |
| Biblioteca | Menos código, más cobertura de casos raros | Dependencia externa, posible `unsafe`, hay que escribir las pruebas de todos modos |
| libpcap del sistema | Máxima compatibilidad | Depende del sistema: rompe la instalación trivial |