/// Contiene las funciones para poder
/// operar con los distintos tipos de
/// bases de datos y funciones creadas
/// para más simplicidad en caso de
/// querer cambiar el tipo de la base
/// de datos; implementando el patrón
/// de diseño Abstract Factory para
/// lograrlo junto con la interfaz
/// Database.
pub mod class;

/// Contiene las operaciones que
/// deberán tener los distintos tipos
/// de bases de datos del aplicativo;
/// logrando así el patrón de diseño
/// Abstract Factory; forzando a que
/// los distintos tipos de bases de
/// datos se implementen
/// correctamente.
/// /// Las bases de datos contienen
/// los datos de los clientes y sus
/// saldos.
pub mod db;

/// Contiene una lista con todos los
/// errores posibles de la aplicación
/// para un manejo más eficiente de
/// los mismos.
/// También posee una función para
/// procesar los mismos de una manera
/// simple.
pub mod error;