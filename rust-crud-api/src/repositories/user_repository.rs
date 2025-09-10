use crate::models::User;
use crate::database::Database;
use postgres::Error as PostgresError;

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn create(&mut self, user: &User) -> Result<(), PostgresError> {
        self.db.get_client().execute(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&user.name, &user.email]
        )?;
        Ok(())
    }

    pub fn find_by_id(&mut self, id: i32) -> Result<Option<User>, PostgresError> {
        let rows = self.db.get_client().query("SELECT id, name, email FROM users WHERE id = $1", &[&id])?;
        
        if rows.is_empty() {
            Ok(None)
        } else {
            let row = &rows[0];
            let user = User::with_id(
                row.get(0),
                row.get(1),
                row.get(2),
            );
            Ok(Some(user))
        }
    }

    pub fn find_all(&mut self) -> Result<Vec<User>, PostgresError> {
        let mut users = Vec::new();
        
        for row in self.db.get_client().query("SELECT id, name, email FROM users", &[])? {
            users.push(User::with_id(
                row.get(0),
                row.get(1),
                row.get(2),
            ));
        }
        
        Ok(users)
    }

    pub fn update(&mut self, id: i32, user: &User) -> Result<u64, PostgresError> {
        let rows_affected = self.db.get_client().execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            &[&user.name, &user.email, &id],
        )?;
        Ok(rows_affected)
    }

    pub fn delete(&mut self, id: i32) -> Result<u64, PostgresError> {
        let rows_affected = self.db.get_client().execute(
            "DELETE FROM users WHERE id = $1",
            &[&id]
        )?;
        Ok(rows_affected)
    }
}
