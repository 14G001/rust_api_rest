use crate::{db::{class::ram_db::singleton::get_db, error::*},
    utils::{date::format::get_curr_date_ddmmyyyy, file::create_file}};

/// Servicio de la API que persiste
/// los datos de todos los clientes y
/// sus balances en un archivo con
/// formato:
/// "DDMMAAAA_<Número de archivo>.DAT"
/// Y luego deja los saldos de todos
/// los clientes con valor 0.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno al intentar generar el archivo.
pub fn generate_result_balances() -> DbError {
    let mut file_content = String::new();
    let ram_db = get_db();
    for (cli_id, cli) in ram_db.clients.iter_mut() {
        let new_line = format!("{:02} {:.2}\n", cli_id, cli.balance);
        cli.balance = 0.0;
        file_content = file_content + &new_line;
    }
    ram_db.file_count += 1;
    create_file(
        format!("{}_{}.DAT", get_curr_date_ddmmyyyy(), ram_db.file_count -1),
        false,
        &file_content
    )
}