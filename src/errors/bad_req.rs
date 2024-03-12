use actix_web::{HttpResponse, body::BoxBody, http::StatusCode};

pub fn bad_request(main_err_msg: &str, err_msg: &str) -> HttpResponse<BoxBody> {
    let mut result_msg = format!("{}{}", main_err_msg, err_msg);
    return HttpResponse::build(StatusCode::BAD_REQUEST).body(result_msg);
}