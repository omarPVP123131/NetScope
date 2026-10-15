# netscope-capture

**Dueño: Persona A (Captura e ingesta).** Nadie más edita este crate.

## Qué vive aquí

Cómo entran los bytes en el programa y qué se hace con ellos después. Es el
único crate que habla con ficheros, con el sistema operativo y con el reloj.

```text
src/
├── lib.rs
├── source.rs       # trait PacketSource: la abstracción común
├── pipeline.rs     # fuente → registro → almacén → estadísticas
├── store.rs        # PacketStore: índice, capacidad, consulta
├── filter.rs       # filtros por protocolo, dirección, puerto y texto
├── stats.rs        # totales, distribución y caudal
├── export.rs       # exportar una selección a .pcap
├── pcap/           # lectura de ficheros
│   ├── file.rs     # PCAP clásico (Sprint 2)
│   └── pcapng.rs   # PCAPNG (Sprint 3)
├── capture/        # captura en vivo, el único código por sistema (Sprint 6)
│   ├── linux.rs    #   feature capture-linux
│   ├── windows.rs  #   feature capture-windows
│   ├── macos.rs    #   feature capture-macos
│   └── unsupported.rs
└── generator.rs    # generador de paquetes para laboratorio (Sprint 7)
```

## Reglas

- **No** interpretar campos de protocolo. Este crate no sabe qué es un número de
  secuencia TCP: pide al registro de `netscope-protocol` que lo interprete y guarda
  el resultado.
- **No** bloquear nunca con un bloqueo abierto mientras se decodifica o se
  captura.
- **No** crecer sin límite: el almacén tiene tope y lo dice en las estadísticas.
- Ningún fichero de captura puede terminar el programa de forma anormal.

## Contrato

[`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md), sección «De `capture` a `desktop`».