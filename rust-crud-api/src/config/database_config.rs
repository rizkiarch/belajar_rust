use std::env;

pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("DATABASE_URL")?;
        Ok(DatabaseConfig { url })
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
