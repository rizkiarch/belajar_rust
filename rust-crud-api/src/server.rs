use crate::controllers::{UserController};
use crate::controllers::user_controller::NOT_FOUND;
use crate::services::UserService;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

pub struct Server {
    listener: TcpListener,
    user_controller: UserController,
}

impl Server {
    pub fn new(address: &str, user_service: Arc<Mutex<UserService>>) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address)?;
        let user_controller = UserController::new(user_service);
        
        Ok(Server {
            listener,
            user_controller,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server berjalan di {}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Gagal menerima koneksi: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let mut request = String::new();

        match stream.read(&mut buffer) {
            Ok(size) => {
                request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

                let (status_line, content) = self.route_request(&request);

                if let Err(e) = stream.write_all(format!("{}{}", status_line, content).as_bytes()) {
                    eprintln!("Failed to write response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Unable to read stream: {}", e);
            }
        }
    }

    fn route_request(&self, request: &str) -> (String, String) {
        match request {
            r if r.starts_with("POST /users") => self.user_controller.create_user(r),
            r if r.starts_with("GET /users/") => self.user_controller.get_user(r),
            r if r.starts_with("GET /users") => self.user_controller.get_all_users(r),
            r if r.starts_with("PUT /users/") => self.user_controller.update_user(r),
            r if r.starts_with("DELETE /users/") => self.user_controller.delete_user(r),
            _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
        }
    }
}
