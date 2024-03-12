/// Contiene las operaciones para poder
/// persistir los datos de los clientes
/// y sus saldos en una base de datos en
/// memoria RAM.
pub mod ram_db;

/// Contiene los mecanismos y estructuras
/// para poder trabajar con una base de
/// datos en memoria RAM con los datos de
/// todos los clientes, el próximo ID de
/// cliente a asignar y el contador de
/// archivos creados desde que se inició
/// el servicio.
/// Este módulo utiliza el patrón de
/// diseño Singleton para poder trabajar
/// con los valores de manera global.
pub mod singleton;