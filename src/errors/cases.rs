use crate::db::error::DbError;

/// Función que devuelve un mensaje de
/// error a partir de un código de error.
///
/// # Argumentos
/// * `err`: Códgio de error.
///
/// # Devuelve
/// Mensaje de error de código de error
/// recibido.
pub fn get_err_msg(err: &DbError) -> String {
    match err {
        DbError::None           => "Unnexpected error",
        DbError::ClientNotExist => "The client does not exist",
        DbError::DocNumAlreadyInUse => "There is already a client with that document number",
        DbError::InvalidFields  => "Invalid field types",
        DbError::CantCreateMoreClients => "Number of clients is at limit",
        DbError::UnknownPayWay  => "Unsupported pay way",
        DbError::FileCreation   => "Error creating file",
        DbError::FileOpening    => "Error opening file",
        DbError::FileReading    => "Error reading file",
        DbError::FileWriting    => "Error writing file",
        DbError::FileExists     => "File already exist",
        _                       => "Unnexpected error"
    }.to_string()
}