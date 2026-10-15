# Licencia

NetScope se distribuye bajo **doble licencia**: puedes elegir la que prefieras.

- [`LICENSE-MIT`](LICENSE-MIT) — MIT License
- [`LICENSE-APACHE`](LICENSE-APACHE) — Apache License 2.0

## Qué significa

Eres libre de usar NetScope, estudiarlo, modificarlo y redistribuirlo. Puedes:

- usarlo en un proyecto personal, sin publicar nada.
- usarlo en un proyecto comercial.
- cambiarlo entero y llamarlo como quieras.
- incluirlo en otro programa.

Las dos licencias dicen casi lo mismo. La diferencia práctica:

| | MIT | Apache-2.0 |
|---|---|---|
| Longitud | 20 líneas | 200 |
| Patents | Sin cláusula explícita | **Cláusula de patentes**: si NetScope se usara para cometer una infracción sobre una patente tuya, la licencia de esa patente se conserva |
| Requisitos al redistribuir | Aviso de copyright | Los mismos, más conservar los avisos de atribución |

**Recomendación:** elige MIT si buscas simplicidad; elige Apache-2.0 si quieres
la protección explícita de patentes y algún día piensas hacer esto grande.

No hay ninguna diferencia entre ambas para un uso educativo: ninguna de las dos
obliga a abrir el código de lo que construyas encima.

## Aviso

Las dos licencias se aplican **tal cual**, sin ninguna cláusula adicional. No
hay código con licencia distinta ni ningún archivo exento: se aplica a todo el
repositorio, incluidos los documentos.

## En el `Cargo.toml`

```toml
license = "MIT OR Apache-2.0"
```

Es la forma que entiende Cargo de la doble licencia: quien use el proyecto
como dependencia elige con qué lo usa.