use crate::models::User;
use crate::database::Database;
use crate::repository::UserRepository;
use crate::utils::{get_id_from_request, get_user_from_request_body};
use std::sync::{Arc, Mutex};

pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
pub const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";

pub struct UserHandler {
    db: Arc<Mutex<Database>>,
}

impl UserHandler {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    pub fn handle_post(&self, request: &str) -> (String, String) {
        match get_user_from_request_body(request) {
            Ok(user) => {
                let mut db = match self.db.lock() {
                    Ok(db) => db,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Database lock error".to_string()),
                };

                match UserRepository::create(&mut *db, &user) {
                    Ok(_) => (OK_RESPONSE.to_string(), "User Created".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to create user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid JSON body".to_string()),
        }
    }

    pub fn handle_get(&self, request: &str) -> (String, String) {
        match get_id_from_request(request).parse::<i32>() {
            Ok(id) => {
                let mut db = match self.db.lock() {
                    Ok(db) => db,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Database lock error".to_string()),
                };

                match UserRepository::find_by_id(&mut *db, id) {
                    Ok(Some(user)) => {
                        match serde_json::to_string(&user) {
                            Ok(json) => (OK_RESPONSE.to_string(), json),
                            Err(_) => (INTERNAL_ERROR.to_string(), "JSON serialization error".to_string()),
                        }
                    }
                    Ok(None) => (NOT_FOUND.to_string(), "User not found".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Database error".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        }
    }

    pub fn handle_get_all(&self, _request: &str) -> (String, String) {
        let mut db = match self.db.lock() {
            Ok(db) => db,
            Err(_) => return (INTERNAL_ERROR.to_string(), "Database lock error".to_string()),
        };

        match UserRepository::find_all(&mut *db) {
            Ok(users) => {
                match serde_json::to_string(&users) {
                    Ok(json) => (OK_RESPONSE.to_string(), json),
                    Err(_) => (INTERNAL_ERROR.to_string(), "JSON serialization error".to_string()),
                }
            }
            Err(_) => (INTERNAL_ERROR.to_string(), "Database error".to_string()),
        }
    }

    pub fn handle_put(&self, request: &str) -> (String, String) {
        let id = match get_id_from_request(request).parse::<i32>() {
            Ok(id) => id,
            Err(_) => return (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        };

        match get_user_from_request_body(request) {
            Ok(user) => {
                let mut db = match self.db.lock() {
                    Ok(db) => db,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Database lock error".to_string()),
                };

                match UserRepository::update(&mut *db, id, &user) {
                    Ok(rows_affected) => {
                        if rows_affected > 0 {
                            (OK_RESPONSE.to_string(), "User updated".to_string())
                        } else {
                            (NOT_FOUND.to_string(), "User not found".to_string())
                        }
                    }
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to update user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid JSON body".to_string()),
        }
    }

    pub fn handle_delete(&self, request: &str) -> (String, String) {
        match get_id_from_request(request).parse::<i32>() {
            Ok(id) => {
                let mut db = match self.db.lock() {
                    Ok(db) => db,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Database lock error".to_string()),
                };

                match UserRepository::delete(&mut *db, id) {
                    Ok(rows_affected) => {
                        if rows_affected > 0 {
                            (OK_RESPONSE.to_string(), "User deleted".to_string())
                        } else {
                            (NOT_FOUND.to_string(), "User not found".to_string())
                        }
                    }
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to delete user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        }
    }
}
