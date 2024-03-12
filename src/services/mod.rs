/// Contiene las funciones de cada
/// servicio del servidor y su
/// respectivo manejos de los posibles
/// mensajes que podrá recibir.
/// Con el módulo Atix Web; cada servicio
/// se implementa con el patrón de diseño
/// Chain of Responsibility.
pub mod resource;

/// Contiene una función que inicia todos
/// los servicios del servidor y su
/// respectivos manejos de los posibles
/// mensajes que podrán recibir.
pub mod init;