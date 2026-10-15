# netscope-protocol

**Dueño: Persona B (Protocolos).** Nadie más edita este crate.

## Qué vive aquí

El modelo canónico de datos y los decodificadores. Es el único crate del
proyecto **sin dependencias internas**: no sabe que existen la captura ni la
interfaz.

```text
src/
├── model.rs        # Packet, Layer, Field, ByteRange, Protocol, LinkType
├── decoder.rs      # el contrato: trait Decoder, LayerContext, Reader
├── registry.rs     # qué protocolo corresponde a qué identificador
├── error.rs        # DecodeError y sus causas
├── hexdump.rs      # volcado hexadecimal y ASCII
├── layers/         # un archivo por protocolo
│   ├── ethernet.rs
│   ├── arp.rs
│   ├── ipv4.rs
│   ├── ipv6.rs
│   ├── tcp.rs
│   ├── udp.rs
│   ├── icmp.rs
│   └── dns.rs
└── flow.rs         # reensamblado de flujos TCP (Sprint 7)
```

## Reglas

- **No** abrir ficheros, ni dibujar, ni tocar la red. Nada de E/S aquí.
- **No** copiar los bytes: los campos guardan un rango dentro de la trama.
- **No** confiar en la entrada: toda lectura pasa por `Reader`, que devuelve
  error en vez de indexar.
- `#![forbid(unsafe_code)]` no es negociable.
- Un protocolo nuevo es un archivo en `layers/` + una línea en el registro.

## Contrato

Lo que los demás crates pueden usar está en
[`../../docs/CONTRATOS.md`](../../docs/CONTRATOS.md). Cualquier cambio aquí que
afecte a esa superficie necesita ADR.