/// Contiene todos los posibles errores de
/// procesamiento de los servicios de la API.
///
/// Las variantes del enum son:
///
/// * `None`           : Error inesperado.
/// * `ClientNotExist` : El cliente no existe.
/// * `DocNumAlreadyInUse` : Ya existe un cliente con ese número de documento.
/// * `InvalidFields`  : Tipo de campos incorrecto.
/// * `CantCreateMoreClients` : Ya se ha alcanzado la cantidad máxima de clientes (el formato del archivo se muestra con IDs de 2 dígitos por lo que el ID máximo sería 99).
/// * `UnknownPayWay`  : Método de pago no soportado.
/// * `FileCreation`   : Error creando archivo.
/// * `FileOpening`    : Error abriendo archivo.
/// * `FileReading`    : Error leyendo archivo.
/// * `FileWriting`    : Error escribiendo archivo.
/// * `FileExists`     : El archivo ya existe.
#[derive(Clone)]
pub enum DbError {
    None,
    ClientNotExist,
    DocNumAlreadyInUse,
    InvalidFields,
    CantCreateMoreClients,
    UnknownPayWay,
    FileCreation,
    FileOpening,
    FileReading,
    FileWriting,
    FileExists
}

/// Recibe el resultado de una operación
/// y indica si salió correctamente o
/// si ocurrió un error en la misma.
///
/// # Argumentos
/// * `db_op_result`: Resultado de la operación.
///
/// # Devuelve
/// Verdadero si la operación fué exitosa y
/// falso en caso de error.
pub fn ok(db_op_result: &DbError) -> bool {
    match db_op_result {
        DbError::None => true,
        _ => false
    }
}