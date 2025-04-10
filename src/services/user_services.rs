use crate::models::user::{NewUser, User};
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_repository::UserRepository;
use diesel::result::Error as DieselError;
use uuid::Uuid;

#[derive(Debug)]
pub enum UserError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
    HashError(String),
}

impl From<DieselError> for UserError {
    fn from(err: DieselError) -> UserError {
        match err {
            DieselError::NotFound => UserError::NotFound("User not found".to_string()),
            _ => UserError::DatabaseError(format!("Database error: {}", err)),
        }
    }
}

pub struct UserService {
    pub repository: UserRepository,
    pub role_repository: RoleRepository,
}

impl UserService {
    pub fn new(repository: UserRepository, role_repository: RoleRepository) -> Self {
        UserService {
            repository,
            role_repository,
        }
    }

    pub fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.repository
            .get_users()
            .map_err(|_| UserError::DatabaseError("Failed to fetch users".to_string()))
    }

    pub fn create_user(&self, mut input: NewUser) -> Result<User, UserError> {
        // Validate role exists
        self.role_repository
            .find_by_id(input.role_id)
            .map_err(|_| {
                UserError::ValidationError(format!("Role with id {} not found", input.role_id))
            })?;

        // Hash password
        input.password = bcrypt::hash(input.password.as_str(), 10)
            .map_err(|e| UserError::HashError(format!("Failed to hash password: {}", e)))?;

        // Create user
        self.repository.create_user(input).map_err(|e| match e {
            DieselError::DatabaseError(_, _) => {
                UserError::DatabaseError("Failed to create user".to_string())
            }
            _ => e.into(),
        })
    }

    pub fn get_user(&self, id: Uuid) -> Result<User, UserError> {
        self.repository.get_user(id).map_err(|e| match e {
            DieselError::NotFound => UserError::NotFound(format!("User with id {} not found", id)),
            _ => UserError::DatabaseError("Failed to fetch user".to_string()),
        })
    }

    pub fn update_user(&self, id: Uuid, input: NewUser) -> Result<User, UserError> {
        // Check if user exists
        let mut user_exist = self
            .repository
            .get_user(id)
            .map_err(|_| UserError::NotFound(format!("User with id {} not found", id)))?;

        // Validate role if changed
        if input.role_id != user_exist.role_id {
            self.role_repository
                .find_by_id(input.role_id)
                .map_err(|_| {
                    UserError::ValidationError(format!("Role with id {} not found", input.role_id))
                })?;
            user_exist.role_id = input.role_id;
        }

        // Update fields if provided
        if !input.name.is_empty() {
            user_exist.name = input.name;
        }
        if !input.email.is_empty() {
            user_exist.email = input.email;
        }
        if !input.password.is_empty() {
            user_exist.password = bcrypt::hash(input.password.as_str(), 10)
                .map_err(|e| UserError::HashError(format!("Failed to hash password: {}", e)))?;
        }

        // Update user
        self.repository
            .update_user(id, user_exist)
            .map_err(|_| UserError::DatabaseError(format!("Failed to update user with id {}", id)))
    }

    pub fn delete_user(&self, id: Uuid) -> Result<User, UserError> {
        // Check if user exists first
        self.get_user(id)?;

        self.repository.delete_user(id).map_err(|e| match e {
            DieselError::NotFound => UserError::NotFound(format!("User with id {} not found", id)),
            _ => UserError::DatabaseError(format!("Failed to delete user with id {}", id)),
        })
    }
}
