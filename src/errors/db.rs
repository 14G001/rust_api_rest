use rusqlite::Error;

pub fn process_query_result(mainErrMsg: String, result: Option<&Error>) -> String {
    if result.is_some() {
        return format!("{}{}", mainErrMsg, result.unwrap())
    }
    "".to_string()
}