use crate::models::User;
use crate::database::Database;
use postgres::Error as PostgresError;

pub struct UserRepository;

impl UserRepository {
    pub fn create(db: &mut Database, user: &User) -> Result<(), PostgresError> {
        db.get_client().execute(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&user.name, &user.email]
        )?;
        Ok(())
    }

    pub fn find_by_id(db: &mut Database, id: i32) -> Result<Option<User>, PostgresError> {
        match db.get_client().query_one("SELECT id, name, email FROM users WHERE id = $1", &[&id]) {
            Ok(row) => {
                let user = User::with_id(
                    row.get(0),
                    row.get(1),
                    row.get(2),
                );
                Ok(Some(user))
            }
            Err(PostgresError::__Nonexhaustive) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn find_all(db: &mut Database) -> Result<Vec<User>, PostgresError> {
        let mut users = Vec::new();
        
        for row in db.get_client().query("SELECT id, name, email FROM users", &[])? {
            users.push(User::with_id(
                row.get(0),
                row.get(1),
                row.get(2),
            ));
        }
        
        Ok(users)
    }

    pub fn update(db: &mut Database, id: i32, user: &User) -> Result<u64, PostgresError> {
        let rows_affected = db.get_client().execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            &[&user.name, &user.email, &id],
        )?;
        Ok(rows_affected)
    }

    pub fn delete(db: &mut Database, id: i32) -> Result<u64, PostgresError> {
        let rows_affected = db.get_client().execute(
            "DELETE FROM users WHERE id = $1",
            &[&id]
        )?;
        Ok(rows_affected)
    }
}
