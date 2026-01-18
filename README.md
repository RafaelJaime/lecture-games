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

# Software de Pruebas de Memoria

Este documento define el **alcance funcional completo** del software de entrenamiento, simulación y competición de pruebas de memoria, incluyendo **las 6 pruebas oficiales** y el **funcionamiento del simulador**.

---

## 1. Pantalla de Inicio

Desde la pantalla inicial el usuario puede seleccionar el modo de uso:

### Modos disponibles

* **Training**
  Entrenamiento libre e individual de todas las pruebas, con total control de parámetros.

* **Test**
  Modo de test completo.
  El programa ejecuta automáticamente las **6 pruebas oficiales**, contabilizando marcas e intentos.
  Se pasa de una prueba a otra sin intervención del usuario hasta finalizar el test.
  En cualquier momento puede activarse la tecla **“Training”**.

* **Groups & Schools**
  Modo de competición para colegios y grupos.
  Pensado para situaciones con **un solo ordenador**, proyectando la misma información a todos los participantes simultáneamente.

* **Exhibition**
  Pruebas de memoria rápida no incluidas en la competición oficial.

### Speed Test y datos del competidor

En los modos **Competition** y **Exhibition**:

* El sistema ejecuta un **Speed Test** para comprobar que el ordenador cumple los requisitos mínimos.
* A continuación se muestran campos opcionales:

  * Número de asiento
  * Título del competidor
  * Nombre
  * Clave de seguridad (introducida por un juez)
* Todos los campos pueden dejarse en blanco.

---

## 2. Funcionamiento del Simulador (Training)

El modo **Training** es el núcleo de práctica y configuración avanzada.

### Entrenamiento con dígitos

* Selección de:

  * Número de dígitos
  * Tipo: **decimales o binarios**
  * Tiempo de exposición (**seconds**)
* Tiempos oficiales de competición: **1 y 4 segundos**
* Posibilidad de **guardar y cargar configuraciones**
* Botones:

  * **GO**: iniciar prueba
  * **Check**: comprobar resultados

### Opciones

* **Fast mode**

  * Desactivado: aparecen celdas en blanco para introducir resultados
  * El sistema corrige automáticamente y marca errores en rojo

* **Calculator**

  * Calculadora de puntuaciones
  * Indica si una marca alcanzaría algún **título de maestría** en competición

---

## 3. Configuración Avanzada (Dígitos)

La sección **Configuration** incluye las siguientes pestañas:

### General

* Número de columnas
* Separación entre filas y columnas
* Distancia entre grupos (jump columns)

### Empty cells

* Oculta dígitos concretos por posición
* Admite listas y rangos
  Ejemplo: `3,5-8` → se ocultan las posiciones 3, 5, 6, 7 y 8

### Empty columns

* Oculta columnas completas según su índice

### Sentinel

* Tras usar “Check”, aparece una flecha que indica dónde comienzan los números en el siguiente intento

### Auto

* Agrupaciones numéricas preestablecidas

### Margin

* Posición de los dígitos en pantalla

### Font

* Tipo de fuente, tamaño y color

### Binary

* Asignación de letras a códigos binarios de 2, 3 o 4 bits
* Optimiza la introducción de resultados
* Configuración guardable

### Move

* Reorganización manual de dígitos mediante arrastre
* Selección múltiple con el ratón
* Opciones:

  * **Unselect all**
  * **Remove manual** (restablece posiciones originales)

---

## 4. Figuras de Colores

### Parámetros

* **Number**: número de figuras (competición: 15)
* **Seconds**: tiempo de exposición por figura
* **Blank time**: tiempo en blanco entre figuras (competición: 0,2 s)
* **Constant time**:

  * Activado: velocidad constante
  * Desactivado: velocidad progresiva (modo competición)

### Consideraciones

* Con tiempos ≤ 1,4 s es obligatorio memorizar **mínimo 10 figuras** para puntuar
* Se recomienda comenzar con **1,75 s**
* Icono visual con tabla orientativa de puntuaciones
* Todas las puntuaciones pueden consultarse en la calculadora

---

## 5. Matrices

### Parámetros

* **Columns / Rows**: tamaño de la matriz
* **Size**: tamaño visual
* **Number**: número de matrices (competición: 12)
* **Showtime**: tiempo visible (competición: 5 s)
* **Blank time**: pausa entre matrices (competición: 1 s)
* **Constant time**:

  * Activado: tiempo fijo
  * Desactivado: velocidad creciente

### Introducción de resultados

* Al finalizar, se abre la **plantilla de matrices**
* Las celdas se pintan en azul mediante clic
* Navegación:

  * Ratón
  * Flechas del teclado

### Configuración específica

* Asignación de letras a códigos de 2, 3 o 4 casillas
* Flecha de dirección:

  * Derecha: orden normal
  * Izquierda: comienza por la última matriz
* **Tabulador**: salta matrices no recordadas
* Punto rojo: matriz pendiente de respuesta

---

## 6. Pruebas Oficiales de Competición

### 6.1 Matrices

* 12 matrices
* 6 minutos
* Sin anotaciones
* 10 puntos por casilla
* 2 intentos, solo cuenta el mejor

### 6.2 Binarios (1 / 4 segundos)

* Secuencias de 0 y 1
* Anotación permitida
* 10 intentos
* Puntúa la mejor marca

### 6.3 Figuras de Colores

* 15 figuras
* Velocidad progresiva
* 3 intentos
* Puntúa la mejor marca

### 6.4 Decimales (1 / 4 segundos)

* Dígitos del 0 al 9
* Anotación permitida
* 10 intentos
* Puntúa la mejor marca

### 6.5 Sistema de Medio Dígito

* 1 segundo: desde 14 dígitos
* 4 segundos: desde 20 dígitos
* Fallo por un solo dígito en intento superior → +0,5
* Nunca se concede un dígito completo no logrado sin error

### 6.6 Exhibición (0,5 segundos)

* Incremento de +0,1 s por fallo de un solo dígito en intento superior
* No oficial

---

## 7. Consideraciones Generales de Desarrollo

* Control preciso de tiempos y progresión automática
* Gestión de intentos y selección de mejor marca
* Interfaces gráficas claras y rápidas
* Validación estricta de errores
* Configuración avanzada persistente
* Uso competitivo, educativo y de exhibición

---

Este README define **todas las reglas, flujos y configuraciones necesarias** para desarrollar un software completo de simulación y competición de memoria.
