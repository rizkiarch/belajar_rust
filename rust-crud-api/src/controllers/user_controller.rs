use crate::services::UserService;
use crate::utils::{get_id_from_request, get_user_from_request_body};
use std::sync::{Arc, Mutex};

pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
pub const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";

pub struct UserController {
    user_service: Arc<Mutex<UserService>>,
}

impl UserController {
    pub fn new(user_service: Arc<Mutex<UserService>>) -> Self {
        Self { user_service }
    }

    pub fn create_user(&self, request: &str) -> (String, String) {
        match get_user_from_request_body(request) {
            Ok(user) => {
                let mut service = match self.user_service.lock() {
                    Ok(service) => service,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Service lock error".to_string()),
                };

                match service.create_user(&user) {
                    Ok(_) => (OK_RESPONSE.to_string(), "User Created".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to create user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid JSON body".to_string()),
        }
    }

    pub fn get_user(&self, request: &str) -> (String, String) {
        match get_id_from_request(request).parse::<i32>() {
            Ok(id) => {
                let mut service = match self.user_service.lock() {
                    Ok(service) => service,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Service lock error".to_string()),
                };

                match service.get_user_by_id(id) {
                    Ok(Some(user)) => {
                        match serde_json::to_string(&user) {
                            Ok(json) => (OK_RESPONSE.to_string(), json),
                            Err(_) => (INTERNAL_ERROR.to_string(), "JSON serialization error".to_string()),
                        }
                    }
                    Ok(None) => (NOT_FOUND.to_string(), "User not found".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Service error".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        }
    }

    pub fn get_all_users(&self, _request: &str) -> (String, String) {
        let mut service = match self.user_service.lock() {
            Ok(service) => service,
            Err(_) => return (INTERNAL_ERROR.to_string(), "Service lock error".to_string()),
        };

        match service.get_all_users() {
            Ok(users) => {
                match serde_json::to_string(&users) {
                    Ok(json) => (OK_RESPONSE.to_string(), json),
                    Err(_) => (INTERNAL_ERROR.to_string(), "JSON serialization error".to_string()),
                }
            }
            Err(_) => (INTERNAL_ERROR.to_string(), "Service error".to_string()),
        }
    }

    pub fn update_user(&self, request: &str) -> (String, String) {
        let id = match get_id_from_request(request).parse::<i32>() {
            Ok(id) => id,
            Err(_) => return (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        };

        match get_user_from_request_body(request) {
            Ok(user) => {
                let mut service = match self.user_service.lock() {
                    Ok(service) => service,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Service lock error".to_string()),
                };

                match service.update_user(id, &user) {
                    Ok(true) => (OK_RESPONSE.to_string(), "User updated".to_string()),
                    Ok(false) => (NOT_FOUND.to_string(), "User not found".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to update user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid JSON body".to_string()),
        }
    }

    pub fn delete_user(&self, request: &str) -> (String, String) {
        match get_id_from_request(request).parse::<i32>() {
            Ok(id) => {
                let mut service = match self.user_service.lock() {
                    Ok(service) => service,
                    Err(_) => return (INTERNAL_ERROR.to_string(), "Service lock error".to_string()),
                };

                match service.delete_user(id) {
                    Ok(true) => (OK_RESPONSE.to_string(), "User deleted".to_string()),
                    Ok(false) => (NOT_FOUND.to_string(), "User not found".to_string()),
                    Err(_) => (INTERNAL_ERROR.to_string(), "Failed to delete user".to_string()),
                }
            }
            Err(_) => (BAD_REQUEST.to_string(), "Invalid user ID".to_string()),
        }
    }
}
