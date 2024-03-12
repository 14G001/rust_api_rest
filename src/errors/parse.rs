use actix_web::{HttpResponse, body::BoxBody};
use crate::errors::bad_req::bad_request;

pub fn parsing_error(err: String) -> HttpResponse<BoxBody> {
    return bad_request("Error parsing json: {:#?}", &err);
}