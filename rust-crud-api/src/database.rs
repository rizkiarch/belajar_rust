use postgres::{Client, NoTls, Error as PostgresError};
use std::env;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Result<Self, PostgresError> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let client = Client::connect(&database_url, NoTls)?;
        
        Ok(Database { client })
    }

    pub fn setup_tables(&mut self) -> Result<(), PostgresError> {
        self.client.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL UNIQUE
            )
            "
        )?;
        Ok(())
    }

    pub fn get_client(&mut self) -> &mut Client {
        &mut self.client
    }
}
