# Hoja de firmas del contrato

> **Esta hoja es un resumen, no el contrato.** El texto que manda es
> [`CONTRATOS.md`](CONTRATOS.md). Si algo de las dos se contradice, manda
> `CONTRATOS.md` y esta hoja está mal: se arregla aquí en el momento.
>
> Para imprimir: tres copias, una por persona, y se firma con fecha.

---

**Proyecto:** NetScope · **Sprint de firma:** 1 · **Fecha:** 19 de octubre de 2026

---

## 1. Reglas de dependencia

- [ ] `netscope-protocol` no depende de ningún crate del proyecto.
- [ ] `netscope-capture` no depende de `netscope-desktop`.
- [ ] `netscope-desktop` no abre ficheros ni habla con el sistema operativo.
- [ ] Lo que otro necesita es `pub`. Nadie usa `pub(crate)` para lo que es
      contrato.
- [ ] Un tipo compartido vive en `netscope-protocol`. Nunca se duplica.
- [ ] `unsafe` está prohibido en los tres crates.

## 2. Promesas de `netscope-protocol` (Persona B)

- [ ] `Packet::data` contiene siempre todos los bytes capturados.
- [ ] Todo `ByteRange` de un campo es relativo al inicio de `packet.data`.
- [ ] `range.slice(datos)` devuelve `None` si no cabe. Nunca hay acceso fuera de
      rango.
- [ ] Un paquete con errores sigue siendo utilizable, con las capas que sí se
      pudieron leer.
- [ ] `decode` no reserva memoria no acotada ni entra en bucle.
- [ ] El nombre de cada campo es estable: `"Puerto origen"`, no `"puerto"`.

## 3. Promesas de `netscope-capture` (Persona A)

- [ ] Los índices de paquete son estables y monotónicos durante una sesión.
- [ ] `ingest` se puede llamar desde el hilo de captura sin bloquear la
      interfaz.
- [ ] El almacén tiene tope de capacidad y lo dice en las estadísticas.
- [ ] `Filter::parse` acepta el mismo lenguaje que se escribe en la caja de
      búsqueda.
- [ ] Un filtro mal escrito es un error de usuario con mensaje, no un pánico.
- [ ] Ningún fichero de captura, por roto que sea, termina el programa.

## 4. Lo que ninguna persona promete

- [ ] Nadie edita código del crate de otra persona.
- [ ] Cambiar un contrato requiere ADR y actualización de `CONTRATOS.md`.
- [ ] Un PR toca un crate, y lo revisa alguien que no lo ha escrito.

---

## 5. Firmas

Al firmar, me comprometo a **coder contra estas firmas aunque la otra persona
todavía no las haya implementado**, y a avisar el mismo día si alguna no me
sirve para lo que tengo que hacer.

| | Nombre | Firma | Fecha |
|---|---|---|---|
| **Persona A** (captura) | | | |
| **Persona B** (protocolos) | | | |
| **Persona C** (interfaz) | | | |

---

## 6. Objeciones del lunes

Si al revisar el contrato alguien tiene una objeción, se escribe aquí antes de
firmar. Una objeción sin resolver bloquea el sprint entero: el objetivo del
Sprint 1 es precisamente no repetir el problema en los once siguientes.

| Quién | Qué promete que no puede cumplir | Por qué | Propuesta |
|---|---|---|---|
| | | | |
| | | | |

**Tres escenarios posibles al terminar este apartado:**

1. Sin objeciones → se firma y se empieza a trabajar.
2. Objeciones menores → se corrigen en el momento, en el documento, y se firma.
3. Una objeción de fondo → **no se firma.** Se resuelve entre las tres personas
   en el día, porque el coste de firmar un contrato que no vale es el Sprint 2
   entero perdido.

---

## 7. Decisiones que NO se toman aquí

Estas no bloquean porque no afectan a la firma:

| Decisión | Cuándo |
|---|---|
| Cuál biblioteca para la captura en vivo | Sprint 10, con un *spike* de dos días |
| Si el generador de paquetes lo hace B o C | Sprint 9, según quién tenga carga |
| Si entran HTTP o TLS | Sprint 12, si todo lo esencial está en verde |
| Si se firma digitalmente el binario | Sprint 13, con presupuesto |