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