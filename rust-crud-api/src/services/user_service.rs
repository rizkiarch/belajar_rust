use crate::models::User;
use crate::repositories::UserRepository;
use postgres::Error as PostgresError;

#[derive(Debug)]
pub enum ServiceError {
    ValidationError,
    DatabaseError,
}

impl From<PostgresError> for ServiceError {
    fn from(_error: PostgresError) -> Self {
        ServiceError::DatabaseError
    }
}

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub fn create_user(&mut self, user: &User) -> Result<(), ServiceError> {
        // Business logic validation could go here
        if user.name.trim().is_empty() {
            return Err(ServiceError::ValidationError);
        }
        
        if user.email.trim().is_empty() || !user.email.contains('@') {
            return Err(ServiceError::ValidationError);
        }

        self.user_repository.create(user).map_err(ServiceError::from)
    }

    pub fn get_user_by_id(&mut self, id: i32) -> Result<Option<User>, ServiceError> {
        if id <= 0 {
            return Ok(None);
        }
        
        self.user_repository.find_by_id(id).map_err(ServiceError::from)
    }

    pub fn get_all_users(&mut self) -> Result<Vec<User>, ServiceError> {
        self.user_repository.find_all().map_err(ServiceError::from)
    }

    pub fn update_user(&mut self, id: i32, user: &User) -> Result<bool, ServiceError> {
        if id <= 0 {
            return Ok(false);
        }

        // Business logic validation
        if user.name.trim().is_empty() {
            return Err(ServiceError::ValidationError);
        }
        
        if user.email.trim().is_empty() || !user.email.contains('@') {
            return Err(ServiceError::ValidationError);
        }

        let rows_affected = self.user_repository.update(id, user).map_err(ServiceError::from)?;
        Ok(rows_affected > 0)
    }

    pub fn delete_user(&mut self, id: i32) -> Result<bool, ServiceError> {
        if id <= 0 {
            return Ok(false);
        }
        
        let rows_affected = self.user_repository.delete(id).map_err(ServiceError::from)?;
        Ok(rows_affected > 0)
    }
}
