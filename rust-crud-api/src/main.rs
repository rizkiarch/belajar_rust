mod models;
mod database;
mod repositories;
mod services;
mod controllers;
mod utils;
mod server;

use database::Database;
use repositories::UserRepository;
use services::UserService;
use server::Server;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};

fn main() {
    // Load environment variables from .env file (optional for Docker)
    dotenv().ok();
    
    // Initialize database
    let mut db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Setup database tables
    if let Err(e) = db.setup_tables() {
        eprintln!("Failed to setup database tables: {}", e);
        return;
    }

    // Initialize repository
    let user_repository = UserRepository::new(db);

    // Initialize service
    let user_service = UserService::new(user_repository);

    // Wrap service in Arc<Mutex<>> for thread safety
    let user_service = Arc::new(Mutex::new(user_service));

    // Create and run server
    let server = match Server::new("0.0.0.0:8080", user_service) {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to create server: {}", e);
            return;
        }
    };

    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
    }
}
