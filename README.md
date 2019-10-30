# Ejemplo de pruebas unitarias con Rust

## RustMX - Meetup octubre 2019

Una de las prácticas que nos ayudan a desarrollar nuestras habilidades son las
'katas'. Las 'katas' son un conjunto de ejercicios orientados a desarrollar
temas específicos, en este caso son las pruebas unitarias lo que queremos
mejorar.

La 'kata' es el ejercicio de FizzBuzz. Se deben imprimir en pantalla los números
del 1 al 100, pero se deben cumplir las siguientes reglas:

* Si el número es múltiplo de 3, en lugar de imprimir el número se imprime
  "Fizz"
* En caso de que el número sea múltiplo de 5, se debe imprimir "Buzz"
* Y finalmente, si el número es múltiplo de 3 y múltiplo de 5, se debe imprimir
  "FizzBuzz"

En este repositorio se incluye el código inicial para completar el ejercicio,
requiere tener instalado el *toolchain* de Rust para poder compilarlo.

Para ejecutar las pruebas unitarias se usa `cargo test` desde la línea de
comandos.

