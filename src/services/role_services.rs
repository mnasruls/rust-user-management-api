use crate::models::role::{NewRole, Role};
use crate::repositories::role_repository::RoleRepository;
use diesel::result::Error as DieselError;
use uuid::Uuid;

#[derive(Debug)]
pub enum RoleError {
    DatabaseError(String),
    NotFound(String),
}

impl From<DieselError> for RoleError {
    fn from(err: DieselError) -> RoleError {
        match err {
            DieselError::NotFound => RoleError::NotFound("Role not found".to_string()),
            _ => RoleError::DatabaseError(format!("Database error: {}", err)),
        }
    }
}

pub struct RoleService {
    pub repository: RoleRepository,
}

impl RoleService {
    pub fn new(repository: RoleRepository) -> Self {
        Self { repository }
    }

    pub fn create_role(&self, role: NewRole) -> Result<Role, RoleError> {
        self.repository.create(role).map_err(|e| match e {
            DieselError::DatabaseError(_, _) => {
                RoleError::DatabaseError("Failed to create role".to_string())
            }
            _ => e.into(),
        })
    }

    pub fn get_role(&self, id: Uuid) -> Result<Role, RoleError> {
        self.repository.find_by_id(id).map_err(|e| match e {
            DieselError::NotFound => RoleError::NotFound(format!("Role with id {} not found", id)),
            _ => RoleError::DatabaseError("Failed to fetch role".to_string()),
        })
    }

    pub fn get_roles(&self) -> Result<Vec<Role>, RoleError> {
        self.repository
            .find_all()
            .map_err(|_e| RoleError::DatabaseError("Failed to fetch roles".to_string()))
    }

    pub fn update_role(&self, id: Uuid, input: NewRole) -> Result<Role, RoleError> {
        let mut role_exist: Role = self
            .repository
            .find_by_id(id)
            .map_err(|_| RoleError::NotFound(format!("Role with id {} not found", id)))?;

        // Update the role fields
        if input.code != role_exist.code {
            role_exist.code = input.code;
        }
        if input.description != role_exist.description {
            role_exist.description = input.description;
        }
        if input.name != role_exist.name {
            role_exist.name = input.name;
        }
        role_exist.updated_at = chrono::Utc::now().naive_utc();

        self.repository
            .update(id, role_exist)
            .map_err(|_e| RoleError::DatabaseError(format!("Failed to update role with id {}", id)))
    }

    pub fn delete_role(&self, id: Uuid) -> Result<Role, RoleError> {
        // First check if role exists
        self.get_role(id)?;

        self.repository.delete(id).map_err(|e| match e {
            DieselError::NotFound => RoleError::NotFound(format!("Role with id {} not found", id)),
            _ => RoleError::DatabaseError(format!("Failed to delete role with id {}", id)),
        })
    }
}
