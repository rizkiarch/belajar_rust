use crate::models::User;
use serde_json;

pub fn get_id_from_request(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

pub fn get_user_from_request_body(request: &str) -> Result<User, serde_json::Error> {
    let body = request
        .split("\r\n\r\n")
        .nth(1)
        .unwrap_or_default();
    
    serde_json::from_str(body)
}
