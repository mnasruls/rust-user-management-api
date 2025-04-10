use crate::config::database::DbPool;
use crate::models::role::{NewRole, Role};
use crate::schema::roles::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct RoleRepository {
    pub pool: DbPool,
}
impl RoleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find_all(&self) -> Result<Vec<Role>, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        roles.load::<Role>(&mut conn)
    }
    pub fn find_by_id(&self, role_id: Uuid) -> Result<Role, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        roles.find(role_id).get_result::<Role>(&mut conn)
    }
    pub fn create(&self, role: NewRole) -> Result<Role, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(roles)
            .values(&role)
            .get_result::<Role>(&mut conn)
    }
    pub fn update(&self, role_id: Uuid, role: Role) -> Result<Role, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(roles.find(role_id))
            .set(&role)
            .get_result::<Role>(&mut conn)
    }
    pub fn delete(&self, role_id: Uuid) -> Result<Role, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::delete(roles.find(role_id)).get_result(&mut conn)
    }
}
