# Cómo colaborar en NetScope

Estas reglas existen para que tres personas puedan trabajar a la vez sin pisarse
y sin que nadie tenga que pedir permiso para seguir.

---

## 1. Cada persona, su crate

| Persona | Crate | Carpeta |
|---------|-------|---------|
| A | `netscope-capture` | `crates/netscope-capture/` |
| B | `netscope-protocol` | `crates/netscope-protocol/` |
| C | `netscope-desktop` | `crates/netscope-desktop/` |

**Nadie edita el crate de otra persona.** Si necesitas algo de su crate:

1. Comprueba si ya está en [`docs/CONTRATOS.md`](docs/CONTRATOS.md).
2. Si está, úsalo y abre una incidencia si parece incorrecto.
3. Si no está, **abre una incidencia pidiendo el cambio de contrato**, no un PR
   que lo incluya de paso.

---

## 2. Ramas

```text
main              # siempre estable, siempre verde
  └─ persona-a/descripcion-corta
  └─ persona-b/descripcion-corta
  └─ persona-c/descripcion-corta
  └─ fix/algo-urgente
```

- Nadie trabaja directamente en `main`.
- Las ramas viven poco: cuanto más cortas, menos dolor.
- Antes de integrar, la rama debe estar verde en integración continua.

---

## 3. Commits

Formato: `tipo(ámbito): descripción en imperativo`.

```text
feat(protocol): añade el decodificador de IPv6 con cabeceras de extensión
fix(capture): no reserva memoria con una longitud leída del fichero
docs(contratos): aclara la semántica de Filter::parse
test(desktop): añade prueba de la selección con almacén vacío
perf(desktop): cachea el ancho de columna de la lista
```

Tipos: `feat`, `fix`, `docs`, `test`, `perf`, `refactor`, `chore`.

- Un commit = un cambio con sentido.
- El cuerpo explica **por qué**, no el qué (el qué ya se lee en el diff).
- Si el cambio altera un contrato, el mensaje lo dice y referencia el ADR.

---

## 4. Pull requests

- **Un PR, un crate, una persona.**
- Por debajo de 400 líneas de cambios. Si no cabe, se parte.
- La descripción dice: qué hace, por qué, cómo se ha probado y qué contratos
  toca.
- Se revisa por la persona que **no** ha escrito el código. Una revisión por
  sprint como mínimo.
- Los PR de documentación de contrato se revisan por las tres personas: si las
  tres están de acuerdo en el papel, ya no habrá sorpresas en el código.

---

## 5. Commits y contratos: el punto delicado

Cambiar la API pública de un crate es un asunto serio:

```text
1. Aviso en el canal el mismo día
2. ADR en docs/adr/
3. PR que cambia el código Y actualiza docs/CONTRATOS.md
4. Un sprint de margen para que los demás adapten su código
```

Excepción: un PR que **cumple** lo prometido pero estaba mal implementado (por
ejemplo, una función que debería devolver un error y entra en pánico) es un
arranque, no una ruptura. Se arregla directamente y se avisa.

---

## 6. Pruebas

Cada cambio llega con pruebas. Como mínimo:

- el caso bueno;
- un caso con entrada hostil (truncado, vacío, incoherente);
- el caso que fallaba antes y ahora pasa (si es un arreglo).

Para decodificadores, la lista de
[`docs/PROTOCOLOS.md`](docs/PROTOCOLOS.md#lista-de-comprobación-de-un-decodificador-nuevo)
es obligatoria.

---

## 7. Fixtures y datos de prueba

- Las capturas `.pcap` **no se suben** al repositorio de forma automática: están
  en `.gitignore`.
- Las que se añadan deben ser del **propio tráfico de quien las aporta**,
  anonimizadas y de menos de 1 MB.
- Antes de añadir una, revisa que no contenga contraseñas, cabeceras de
  autenticación, nombres de usuarios, direcciones internas identificables ni
  dominios privados.
- Las tramas mínimas escritas a mano (unos pocos bytes) sí se versionan: son
  material de pruebas, no tráfico real.

---

## 8. Integración continua

`.github/workflows/ci.yml` corre en cada PR y en cada `main`:

| Job | Qué comprueba | Dónde |
|-----|---------------|-------|
| `rapido` | `cargo fmt --check`, `cargo clippy -D warnings` y `cargo test` de los tres crates, sin interfaz | Cada commit, en segundos |
| `interfaz` | Que la interfaz gráfica compila con todos sus features | Antes de integrar cambios de interfaz |
| `sistemas` | Que el núcleo compila en Linux, Windows y macOS | Cada commit, en paralelo |
| `fixtures` | Las capturas de prueba siguen dando el resultado esperado | Aparece en gris hasta que haya fixtures |

Las tres personas deben tener `main` en verde. Si un job falla, se arregla en el
PR: nadie combina con `main` en rojo.

Los `lints` vienen del workspace (`Cargo.toml` > `[workspace.lints]`), así que
`unsafe` está prohibido y la documentación es obligatoria en los tres crates
sin repetir configuración. Para reproducirlos en local:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test --workspace --no-default-features
```

---

## 9. Cuando algo bloquea

1. **Aviso inmediato** en el canal compartido. No se espera a la reunión del
   viernes.
2. Se escribe qué falta exactamente y qué se ha intentado ya.
3. Si es un problema de contrato, se propone la frase exacta que falta en
   `docs/CONTRATOS.md`.
4. Se busca una salida con un doble de prueba mientras tanto. **Esperar no es una
   solución.**

---

## 10. Lo que no se acepta

- `unsafe` en cualquier crate.
- Cambiar un contrato en un PR de otra cosa.
- Un PR que toca los tres crates.
- Un `.pcap` con datos de terceros.
- Código sin pruebas cuando toca lógica de protocolo, filtros o captura.
- Funciones de 200 líneas con cuatro responsabilidades dentro.

Si hay que romper alguna de estas reglas, se escribe un ADR explicando por qué
merece la pena, y se acepta en la revisión.