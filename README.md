# Superlectura Games

This project is developed in **Rust** using the `eframe` and `egui` libraries to create a native graphical interface.

## About the project

The implemented games are based on **Tony Buzan's** techniques and his book on speed reading. Tony Buzan was a British educational psychologist known for popularizing mind maps and developing techniques to improve memory, speed reading, and accelerated learning.

## How to run

### Run the program:
```bash
cargo run
```

### Build the project:
```bash
cargo build
```

### Build in release mode (optimized):
```bash
cargo build --release
```

# README – Software de Pruebas de Memoria

Este software implementa **6 pruebas oficiales de memoria**, orientadas a competición, con reglas estrictas de tiempo, puntuación y número de intentos. A continuación se describen de forma resumida para su desarrollo e implementación.

---

## 1. Matrices

**Descripción**
Memorización visual de matrices formadas por casillas blancas y azules.

**Reglas**

* 12 matrices por intento.
* Duración total: 6 minutos.
* Todas las matrices tienen el mismo número de casillas (elegido por el competidor).
* La velocidad de paso es automática y aumenta progresivamente.
* No se permite ningún tipo de anotación.
* Al finalizar, el usuario debe dibujar todas las matrices en la plantilla del software.

**Puntuación**

* 10 puntos por casilla correctamente memorizada.
* La puntuación depende del número de matrices correctas y del tamaño de cada matriz.
* 2 intentos totales, solo cuenta la mejor marca.

---

## 2. Binarios (1 y 4 segundos)

**Descripción**
Memorización de secuencias de números binarios (0 y 1).

**Reglas**

* Tiempo de exposición: 1 segundo o 4 segundos.
* Tras desaparecer la secuencia, el usuario debe introducirla en orden correcto.
* Se permite anotación en papel antes de escribir la respuesta.
* 10 intentos totales.

**Puntuación**

* Solo puntúa la mejor marca obtenida.

---

## 3. Figuras de Colores

**Descripción**
Memorización de una secuencia de figuras geométricas con color.

**Reglas**

* 15 figuras por intento.
* Duración total: 3 minutos.
* Velocidad inicial elegida por el competidor; luego aumenta automáticamente.
* Con velocidad lenta (1,75 s) puntúa cualquier número de aciertos.
* Con velocidades más rápidas es obligatorio memorizar al menos 10 figuras para puntuar.
* Se permite anotación en papel.
* Al finalizar, las figuras se marcan en una plantilla del software.

**Figuras posibles**

* Círculo, cuadrado, rectángulo, triángulo, pentágono, hexágono, heptágono, estrella.

**Colores posibles**

* Blanco, negro, marrón, rojo, verde, amarillo, naranja, azul, gris, violeta.

**Puntuación**

* Depende de la velocidad inicial y del número de figuras correctas.
* 3 intentos totales, solo cuenta la mejor marca.

---

## 4. Decimales (1 y 4 segundos)

**Descripción**
Memorización de secuencias de dígitos decimales (0–9).

**Reglas**

* Tiempo de exposición: 1 segundo o 4 segundos.
* Tras desaparecer la secuencia, el usuario debe escribirla correctamente.
* Se permite anotación previa en papel.
* 10 intentos totales.

**Puntuación**

* Solo puntúa la mejor marca obtenida.

---

## 5. Sistema de Medio Dígito (Decimales y Binarios)

**Descripción**
Sistema de puntuación parcial para intentos fallidos por un solo dígito.

**Reglas**

* En la prueba de 1 segundo: a partir de 14 dígitos.
* En la prueba de 4 segundos: a partir de 20 dígitos.
* Si el competidor:

  1. Consigue una marca base (ej. 14 dígitos),
  2. Y luego falla un intento superior por un solo dígito,
* Se le añade **+0,5 dígitos** a la puntuación final.
* No se conceden dígitos completos no logrados sin error.

**Ejemplo**

* 14 correctos → intento de 15 fallado por 1 → puntuación final: **14,5**.

---

## 6. Prueba de Exhibición (0,5 segundos)

**Descripción**
Variante rápida para exhibiciones.

**Reglas**

* Cada fallo por un solo dígito en un intento superior incrementa la puntuación en **0,1 segundos**.
* No afecta a las pruebas oficiales, solo a exhibición.

---

## Consideraciones Generales de Desarrollo

* Control automático y progresivo de tiempos.
* Gestión de intentos y almacenamiento de la mejor marca.
* Plantillas gráficas para reproducción (matrices y figuras).
* Validación estricta de errores (especialmente para medio dígito).
* Registro claro de puntuación final por prueba.

---

Este README define las reglas funcionales mínimas necesarias para implementar el software de entrenamiento y competición de memoria.
