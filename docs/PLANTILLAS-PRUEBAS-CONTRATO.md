# Plantillas de las pruebas de contrato

> Las pruebas de contrato se escriben **antes que las implementaciones**, el día
> 1 del Sprint 1. Cada persona prueba lo que la otra promete, no lo que la otra
> ha hecho.
>
> **Regla de oro:** si una prueba de contrato falla, **el problema está en el
> contrato**, no en la persona que aún no lo ha implementado. Por eso las
> pruebas van en un sitio aparte y se ejecutan aunque la implementación no
> exista.

---

## Dónde viven

```text
crates/netscope-capture/tests/contrato_con_protocol.rs    # A prueba lo que promete B
crates/netscope-desktop/tests/contrato_con_capture.rs     # C prueba lo que promete A
crates/netscope-protocol/tests/contrato_con_capture.rs    # B prueba lo que promete A
```

Cada archivo tiene la cabecera obligatoria:

```rust
//! Pruebas de contrato de `netscope-capture`.
//! Comprobamos lo prometido en docs/CONTRATOS.md, no lo implementado.
//! Si una falla, se arregla el contrato o la implementación, no la prueba.
#![cfg(feature = "triage")]   // hasta que exista la implementación
```

La última línea es lo que permite tener las pruebas escritas desde el día 1 sin
que fallen la CI: se activan cuando la implementación está.

---

## 1. A prueba lo que promete B (`contrato_con_protocol.rs`)

```rust
use netscope_protocol::{decode, LinkType, Protocol};

/// Promesa: "todo ByteRange de un campo es relativo al inicio de packet.data"
#[test]
fn los_offsets_son_relativos_al_inicio_de_la_trama() {
    let trama = trama_ethernet_ipv4_udp();           // de fixtures/minimos
    let paquete = decode(&trama, LinkType::Ethernet);

    let ip = paquete.layer(Protocol::Ipv4).expect("capa IPv4");
    let origen = ip.field("Dirección origen").expect("campo");

    // El offset 26 es 14 (cabecera Ethernet) + 12 (dentro de IPv4).
    assert_eq!(origen.range.offset, 26);
    // Y el rango se puede leer de verdad contra el mismo buffer.
    assert_eq!(
        origen.range.slice(&paquete.data),
        Some(&[10, 0, 0, 1][..])
    );
}
```

```rust
/// Promesa: "un paquete con errores sigue siendo utilizable"
#[test]
fn un_paquete_truncado_sigue_teniendo_capas() {
    let trama = trama_ipv4_truncada();               // de fixtures/minimos
    let paquete = decode(&trama, LinkType::Ethernet);

    assert!(!paquete.errors.is_empty(), "debe avisar del truncamiento");
    assert!(paquete.layer(Protocol::Ethernet).is_some(), "y seguir siendo legible");
    assert!(paquete.layer(Protocol::Ipv4).is_some());
}
```

```rust
/// Promesa: "decode no entra en bucle" (ADR de seguridad)
#[test]
fn un_ethertype_que_se_apunta_a_si_mismo_termina() {
    // EtherType 0x0800 dentro de un paquete IPv4 mal formado, repetido.
    let trama = trama_con_ethertype_recursivo();
    let paquete = decode(&trama, LinkType::Ethernet);

    assert!(paquete.layers.len() < 20, "no debe descifrar sin fin");
}
```

```rust
/// Promesa: "el nombre de cada campo es estable"
#[test]
fn los_nombres_de_campo_son_los_prometidos() {
    let paquete = decode(&trama_ethernet_ipv4_udp(), LinkType::Ethernet);
    let nombres: Vec<&str> = paquete
        .layers
        .iter()
        .flat_map(|c| c.fields.iter())
        .map(|f| f.name)
        .collect();

    for esperado in ["Dirección origen", "Dirección destino", "TTL",
                     "Longitud total", "Puerto origen", "Puerto destino"] {
        assert!(nombres.contains(&esperado), "falta el campo {esperado}");
    }
}
```

---

## 2. C prueba lo que promete A (`contrato_con_capture.rs`)

```rust
use netscope_capture::{Filter, PacketSource, Stats, Store};

/// Promesa: "los índices son estables y monotónicos, aunque el almacén descarte"
#[test]
fn los_indices_no_se_reutilizan_al_descartar() {
    let mut store = Store::con_capacidad(10);
    for i in 0..30 {
        store.insert(paquete_de_prueba(i));
    }

    let indices: Vec<u64> = store.iter().map(|p| p.index).collect();
    assert!(indices.windows(2).all(|w| w[1] > w[0]), "deben ser crecientes");
    assert_eq!(store.descartados(), 20, "el almacén lo dice");
}
```

```rust
/// Promesa: "un filtro mal escrito es un error con mensaje, no un pánico"
#[test]
fn un_filtro_mal_escrito_explica_el_problema() {
    let error = Filter::parse("puerto 443x").expect_err("debe fallar");

    let mensaje = error.to_string();
    assert!(mensaje.contains("443x"), "el mensaje cita lo que no entiende: {mensaje}");
}
```

```rust
/// Promesa: "el mismo lenguaje que se escribe en la caja de búsqueda"
#[test]
fn el_filtro_hace_lo_que_promete_con_un_ejemplo_concreto() {
    let captura = captura_de_ejemplo();          // 100 paquetes conocidos
    let visibles = Filter::parse("tcp && port 443").unwrap()
        .aplicar(&captura).count();

    assert_eq!(visibles, 37, "el número exacto está escrito en fixtures");
}
```

```rust
/// Promesa: "el almacén no crece sin límite y lo dice"
#[test]
fn la_capacidad_es_un_tope_real() {
    let mut store = Store::con_capacidad(100);
    for _ in 0..10_000 {
        store.insert(paquete_de_prueba(0));
    }

    assert!(store.len() <= 100);
    assert!(store.stats().memoria_acotada());
}
```

---

## 3. B prueba lo que promete A (`contrato_con_capture.rs`)

```rust
use netscope_capture::PacketSource;

/// Promesa: "un .pcap truncado no termina el programa"
#[test]
fn un_fichero_de_captura_cortado_a_mano_se_lee_hasta_el_final() {
    let completo = std::fs::read("fixtures/reales/captura-pequena.pcap").unwrap();
    // Se corta por la mitad, en mitad de un registro de paquete.
    let cortado = &completo[..completo.len() / 2];

    let mut source = netscope_capture::PcapReader::from_bytes(cortado).unwrap();
    let leidos = source.leer_todo();

    // No entra en pánico y devuelve lo que había.
    assert!(!leidos.is_empty());
    assert!(leidos.iter().all(|p| p.data.len() <= 65535));
}
```

---

## Cuándo se activan

| Momento | Qué se ejecuta |
|---------|----------------|
| Sprint 1, días 1-3 | Se escriben, con `#![cfg(feature = "triage")]` |
| Cuando hay implementación | Se quita el `cfg` de cada bloque, uno a uno |
| Final de cada sprint | Todas, sin excepciones |
| En cada PR que toque un contrato | Todas |

## Reglas

1. **Una prueba por promesa.** Si una promesa no tiene prueba, no está prometida.
2. **El número va escrito en la prueba.** `assert_eq!(visibles, 37)`, no
   `assert!(visibles > 0)`. Una prueba que acepta cualquier número no prueba
   nada.
3. **No se modifican las pruebas para que pasen.** Si una promesa es
   incorrecta, se cambia el contrato, se escribe el ADR y se actualizan las
   pruebas. Al revés nunca.
4. **Una prueba rota el lunes por la mañana es una buena noticia**: ha detectado
   un problema de contrato en 24 horas en vez de en tres semanas.