# Revisión crítica de la planificación

> **14 de octubre de 2026.** Revisión hecha **después** de escribir los
> documentos, buscando lo que falta, lo que se contradice y lo que parece
> tranquilo y no lo es.
>
> No es una lista de errores de quien escribió los documentos: son huecos reales
> que, si no se cierran, aparecen como sorpresas en el Sprint 3 o en el 6.

**Estado de cada hallazgo:**

| Tipo | Significado |
|------|-------------|
| 🔴 **Bloqueante** | Si no se decide, para el trabajo de alguien |
| 🟡 **Importante** | No para, pero se nota al final del sprint |
| ⚪ **Menor** | Se puede decidir sobre la marcha |

---

## 1. Lo que más me preocupa

### 🔴 La Persona B tiene el triple de trabajo que los demás

Esto es el riesgo estructural del reparto, y no está tratado en ningún
documento.

| Persona | Módulos | Sprints |
|---------|---------|---------|
| A | 6 | bajo carga en el 6 |
| B | **15** (modelo, registro, 9 decodificadores, reensamblado) | **cargada de 1 a 8** |
| C | 9 | baja en el 6 |

El Sprint 1 está equilibrado porque lo que se hace es diseñar, no implementar.
Pero desde el Sprint 2, **B es el cuello de botella de todo el proyecto**: si B
no termina TCP en el Sprint 3, el filtro por protocolo, las estadísticas por
protocolo y la vista de detalle se quedan a medias en A y en C.

Lo que agrava el dato: en los ADRs, los documentos y el README, **el trabajo de
B es invisible** porque va dentro de un crate que no se ve en las demos
semanales. A y C enseñan ventanas y listas; B enseña tramas de seis bytes.

**Qué hacer:** en la reunión del Sprint 1, decidir cuál de estas se aplica:

1. Mover DHCP a la Persona C (tiene el menor pico: el 8).
2. Mover el generador de paquetes a la Persona C (es la parte más visual del 7).
3. Mover el reensamblado TCP a un Sprint 9 fuera del MVP.
4. Aceptar que el Sprint 5 sea el de menos alcance del año.

**Recomendación:** la 2 y la 3. El generador va con la interfaz porque es
«lo que se ve», y el reensamblado es la característica más grande del MVP con
menos valor didáctico inmediato para el resto.

### 🔴 Nadie ha estimado el esfuerzo

El roadmap dice qué, no cuánto. 8 sprints × 3 personas es una fecha de
finalización, no una planificación: **es una apuesta** feita sobre la
estimación que cada uno haga en su cabeza el Sprint 1.

Sin una estimación, el primer sprint que va mal no se detecta: se descubre en el
4, cuando ya no se puede recuperar.

**Qué hacer:** en el Sprint 1, el viernes, cada persona estima su Sprint 2 en
puntos (1 = un día, 2 = medio día, 3 = tres días) y se comparan las tres
estimaciones con lo que de verdad costó el Sprint 1. Si alguien se va a
desviar más de 20 %, se recorta alcance ahí, no en el mes de enero.

---

## 2. Contradicciones entre documentos

Encontradas y **corregidas**:

| Contradicción | Dónde | Resolución |
|---------------|-------|------------|
| El rendimiento se mide en el Sprint 4 o en el 6 | `ARQUITECTURA.md` decía 6; `PERSONA-A` decía 4 | **Sprint 4**, en ambos |
| Los crates se llamaban de tres maneras | Tablas, diagramas y texto | Todo unificado en el commit del renombrado |
| La licencia «se decidiría luego» | README frente a la decisión tomada | **MIT OR Apache-2.0**, aplicado y documentado |

**No corregidas a propósito:**

| Contradicción | Por qué se deja |
|---------------|-----------------|
| `docs/PROTOCOLOS.md` describe el orden de los protocolos de forma más detallada que el roadmap | No es una contradicción: uno es el plan y el otro la guía de estudio. Si el roadmap y la guía acaban diciendo cosas distintas, se ajusta la guía |

---

## 3. Huecos reales

### 🔴 No hay política para las pruebas doradas

`fixtures/dorados/` está descrito como «trama → descripción esperada en JSON»,
pero no está dicho:

- ¿Quién lo regenera? (B, con una herramienta)
- ¿Cómo se regenera sin perder el control? (el golden **nunca** se regenera
  automáticamente: se regenera a mano y el diff se lee)
- Qué pasa cuando cambia un nombre de campo, como «Longitud total» a
  «Longitud total del paquete»?

Ese último caso es el peligroso: **cambiar un nombre de campo rompe todos los
doracos de golpe**, y la reacción instintiva es regenerarlos sin mirar. Eso
destruye exactamente la señal que la prueba existe para dar.

**Qué hacer:** las pruebas doradas se versionan **con el número de versión del
formato** en el nombre (`golden-v1.json`). Cuando B cambia algo del modelo, sube
a `v2` en un PR aparte, y el diff entre `v1` y `v2` es lo que se revisa. El
formato solo cambia con ADR.

### 🟡 No está decidido dónde viven los textos de la interfaz

La interfaz va en español (decidido: los identificadores en inglés), pero no se
ha dicho si los textos están:

1. **Directos en el código** (una función por texto: la más rápida).
2. En un fichero de cadenas (permite cambiarlos sin tocar código).
3. Con un sistema de traducción completo (una dependencia más y trabajo que el
   proyecto no necesita).

El riesgo es concreto: si C escribe los textos en las vistas y luego hay que
cambiarlos, hay que tocar veinte archivos.

**Qué hacer:** un único `views/textos.rs` con las cadenas de la interfaz, aunque
al principio tenga diez líneas. Cuesta diez minutos ahora y dos horas después.

### 🟡 El modelo no guarda de qué interfaz viene un paquete

En PCAPNG un fichero puede tener paquetes de varias interfaces, cada una con su
tipo de enlace. El modelo (`Packet`) tiene `link_type`, que es lo justo, pero
**no tiene el identificador de la interfaz**.

No es un problema para el MVP (casi siempre todos los paquetes del mismo
fichero son de la misma interfaz), pero en cuanto se exporte un subconjunto a un `.pcap` que tenga que
reconstruir la cabecera, el dato estará en un sitio imposible de recuperar.

**Qué hacer:** B añade un campo `interface: Option<u16>` al modelo en el
Sprint 1, mientras congelarlo. Añadirlo después es un cambio de contrato.

### ⚪ El timestamp del PCAP clásico tiene fecha de caducidad

La cabecera del PCAP clásico usa enteros de 32 bits para los segundos: **se
queda atrás en 2038**. Es un detalle que no importa hoy, pero es de las cosas
que se dureza una hora el día que aparece un fichero raro.

**Qué hacer:** nota en el código, sin más. La conversión a `u64` se hace al
leer, así que el día que aparezca un fichero con ese problema, es un `as u64` en
un sitio concreto.

---

## 4. Riesgos que no estaban en la lista

| Riesgo | Por qué no se había visto | Qué hacer |
|--------|---------------------------|-----------|
| **B se queda sin contexto y escribe código que solo él entiende** | El trabajo de B es el menos visible: no hay ventana que enseñarle a nadie | Rotación: quien revise un decodificador de B tiene que ser alguien que pueda ejecutarlo y ver sus pruebas. En la práctica, A o C |
| **El contrato se congela demasiado pronto** | Se firmó en el Sprint 1, antes de tener el primer decodificador escrito | Está escrito como «cambiarlo requiere ADR», no «no se puede cambiar». El primer cambio legítimo genera su ADR y el proceso se demuestra |
| **`unsafe` prohibido y `missing_docs` como error, y el primer día no hay quien documentar** | Los lints seemed una buena idea sin mirar el coste real | Los lints están en `warn`, no en `deny`. El CI usa `-D warnings`, así que hay que subirlos a error solo cuando el código esté |
| **El Sprint 5 y el 6 caen en Navidad y Año Nuevo** | El calendario se calculó al final | Ya está avisado en el roadmap. La decisión de bajar alcance se toma en el Sprint 3, no en diciembre |

---

## 5. Lo que está bien y no hay que tocar

- **La independencia entre equipos está bien resuelta.** El DAG es lineal, los
  contratos están escritos antes de codificar y hay tres mecanismos (dobles de
  prueba, fixtures compartidas, pruebas de contrato) para que nadie espere. Es
  lo más difícil de acertar en un proyecto en equipo y aquí está bien.
- **La honestidad sobre el estado.** El README dice claramente que no hay
  ninguna funcionalidad. En un repositorio nuevo, eso evita la mitad de los
  malentendidos.
- **El parser hostil desde el día uno.** `#![forbid(unsafe_code)]`, lints
  heredados del workspace y corpus de tramas mutadas en el Sprint 2. La mayoría de los
  analizadores con fallos de seguridad hacen exactamente lo contrario.

---

## 6. Resumen: qué decidir en la reunión del Sprint 1

| # | Decisión | Quién |
|---|----------|-------|
| 1 | **Reparto de la carga de B** (mover DHCP, generador o reensamblado) | las tres |
| 2 | **Estimación del Sprint 2** en puntos, para poder comparar con el real | las tres |
| 3 | **Política de los dorados** (versionado del formato) | B, con visto bueno de A y C |
| 4 | **Dónde viven los textos de la interfaz** | C |
| 5 | **Campo `interface` en el modelo**, antes de congelarlo | B |
| 6 | **Si el Sprint 5 baja alcance** por las fiestas | las tres |
| 7 | **Reparto del Sprint 8** (reensamblado TCP), que recae entero sobre B | las tres |

Nada de esto para el trabajo del día a día. Todo esto para que la primera
semana no haya sorpresas.

---

## 7. Actualización del 14 de octubre

Tras conocer la capacidad real (**15–20 h semanales por persona**), la revisión
cambia en tres puntos:

- **El roadmap crece** de 8 a 12 sprints: había margen de sobra y el objetivo es
  comparable a Wireshark, no un visor de cabeceras.
- **El riesgo de «nadie estima» baja de bloqueante a importante**: hay horas de
  sobra, pero sin estimación el fallo sigue detectándose tarde.
- **El cuello de botella de B se mueve**: con más horas, el problema ya no es el
  volumen total, sino el **Sprint 8 entero**, donde el reensamblado TCP recae
  solo sobre B. Si ahí se atasca, arrastra los dos siguientes. Por eso está
  aislado y por eso el punto 7 de la lista de arriba.