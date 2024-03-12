/// Contiene una estructura con los datos
/// que pueden tener los clientes y
/// una operación para poder crear los
/// mismos de manera más sencilla.
pub mod client;

/// Contiene estructuras y funciones para
/// la persistencia de los datos de los
/// clientes y sus balances en archivos y
/// en memoria en una base de datos.
/// Implementa el patrón de diseño Abstract
/// Factory para poder reemplazar los
/// métodos que pueden llevarse a cabo
/// con la base de datos de manera más
/// rápida y eficiente a través del trait
/// Database (por ejemplo; cambiando el
/// tipo de la base de datos de una que
/// funciona con la memoria RAM a una que
/// funciona con un servidor externo
/// que trabaja con consultas SQL).
pub mod db;

/// Contiene la estructura en la que se
/// persistirán las variables de entorno y
/// la función con la que se leerán y
/// validarán las mismas.
pub mod env_var;

/// Contiene los posibles errores que se
/// podrán manejar por el aplicativo y
/// modularizaciones para simplificar la
/// devolución de respuestas de error por
/// la API.
pub mod errors;

/// Contiene funciones para iniciar todos
/// los servicios del servidor y su
/// respectivos manejos de los posibles
/// mensajes que podrán recibir.
/// Con el módulo Atix Web; cada servicio
/// se implementa con el patrón de diseño
/// Chain of Responsibility.
pub mod services;

/// Contiene funciones para generar los
/// archivos de balances por clientes y
/// el contador de archivos.
pub mod statistics;

/// Contiene la estructura con los datos
/// de la transacción y funciones para
/// el procesamiento de la misma.
pub mod transaction;

/// Contiene funciones de ayuda para simplificar:
/// * La obtención de la fecha actual en ciertos formatos.
/// * La conversión de mensajes recibidos en un formato a una estructura de la aplicación.
/// * Operaciones para el trabajo con archivos de manera más sencilla.
pub mod utils;