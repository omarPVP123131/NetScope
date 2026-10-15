# ADR-0002 · Un único modelo canónico, con referencias y no con copias

**Estado:** propuesta · **Fecha:** Sprint 1 · **Personas afectadas:** las tres

## Contexto

Un analizador muestra la misma información dos veces: en el panel de campos y en
la vista de bytes. Si cada vista guardara su propia copia de los datos, un
paquete de 1.500 bytes con muchas capas consumiría varias veces más de lo
necesario, y con 100.000 paquetes en memoria la diferencia pasa de "va rápido" a
"va lento".

## Decisión

1. Existe **un solo modelo canónico**, en `netscope-protocol`. La interfaz, los
   filtros y las estadísticas trabajan siempre sobre él.
2. Un `Packet` **es dueño de su buffer** de bytes. Un `Field` **no copia**: guarda
   un rango (`offset`, `len`) dentro de ese buffer.
3. El acceso a los bytes de un campo **siempre pasa por un lector con límites**,
   que devuelve `Option` o error. Nunca se indexa directamente.
4. Un paquete con problemas **no se descarta**: se guarda con la lista de
   errores y se muestra marcado.

```text
Packet { data: Vec<u8> }          ← una sola copia por paquete
  └─ Layer { range, fields[] }
       └─ Field { range }         ← apunta a data, no lo duplica
```

## Consecuencias

**A favor**

- Abrir y filtrar 100.000 paquetes cabe de sobra en memoria moderna.
- La vista de bytes y el panel de campos siempre coinciden: leen el mismo buffer.
- Mostrar un paquete truncado no requiere casos especiales en la interfaz.
- La decodificación es una función pura: se puede paralelizar sin esfuerzo.

**En contra**

- Un `Packet` no puede sobrevivir a su buffer. Quien lo guarde debe guardar
  también los bytes. Se documenta en el contrato.
- Los rangos exigen atención al serializar (por ejemplo, al exportar JSON). Cada
  exportación incluye los bytes o los recalcula.

## Alternativas descartadas

- **Copiar los valores a `String`**: simple, pero multiplica la memoria y
  pierde los tipos (un puerto es un número, no un texto).
- **Referencias con vida producida** (`&'a [u8]` por todo el modelo): más seguro
  en apariencia, pero obliga a que el almacén y la vista compartan vida, lo que
  bloquea a la interfaz y complica los hilos.
- **Acceso por índice de campo**: `fields[3]` se rompe en cuanto alguien añade
  un campo. Se busca siempre por nombre.