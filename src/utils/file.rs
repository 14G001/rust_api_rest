use {std::{fs::{metadata, File}, io::{Read, Write}},
    std::fs::OpenOptions, crate::db::error::DbError};

/// Verifica si el archivo recibido existe o no.
///
/// # Argumentos
/// * `path`: Ruta al archivo.
///
/// # Devuelve
/// Verdadero si el archivo existe; falso en el caso contrario.
pub fn file_exists(path: &str) -> bool {
    let meta = metadata(path);
    match meta {
        Ok(meta) => meta.is_file(),
        _ => false
    }
}

/// Crea un archivo en el directorio recibido
/// con el contenido indicado.
///
/// # Argumentos
/// * `path`: Ruta al archivo.
/// * `overwrite`: Indica si en caso de que el archivo exista se debe sobreescribirlo o indicar un error.
/// * `content`: Contenido a escribir en el archivo.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno al intentar crear el archivo.
pub fn create_file(path: String, overwrite: bool, content: &str) -> DbError {
    if !overwrite && file_exists(&path) {
        return DbError::FileExists;
    }
    match File::create(path) {
        Ok(file) => {
            let mut f_mut = file;
            append_file(&mut f_mut, content.to_string())
        },
        _ => DbError::FileCreation
    }
}

/// Abre un archivo con permisos de
/// lectura o escritura segun sea
/// indicado.
///
/// # Argumentos
/// * `path`: Ruta al archivo.
/// * `read`: Indica si se abrirá con permisos de lectura o no.
/// * `write`: Indica si se abrirá con permisos de escritura o no.
///
/// # Devuelve
/// Archivo abierto.
pub fn open_file(path: &str, read: bool, write: bool) -> File {
    OpenOptions::new().read(read).write(write).open(path).unwrap()
}

/// Escribe datos al final de un archivo.
///
/// # Argumentos
/// * `file`: Archivo en el que escribir los datos.
/// * `content`: Contenido a escribir en el archivo.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno al intentar escribir el archivo.
pub fn append_file(file: &mut File, content: String) -> DbError {
    match file.write_all(&content.as_bytes()) {
        Ok(_file) => DbError::None,
        _ => DbError::FileWriting
    }
}

/// Reemplaza el contenido de un archivo
/// por el contenido recibido.
///
/// # Argumentos
/// * `path`: Archivo a reemplazar el contenido.
/// * `content`: Contenido a escribir en el archivo.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno al intentar reemplazar los datos del
/// archivo.
pub fn overwrite_file(path: &str, content: String) -> DbError {
    return create_file(path.to_string(), true, &content)
}

/// Lee el contenido de un archivo.
///
/// # Argumentos
/// * `content`: Contenido del archivo a leer.
/// * `file`: Archivo a leer.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno al intentar leer los datos del
/// archivo.
pub fn read_file(content: &mut String, file: &mut File) -> DbError {
    match file.read_to_string(content) {
        Ok(_result) => DbError::None,
        _ => DbError::FileReading
    }
}