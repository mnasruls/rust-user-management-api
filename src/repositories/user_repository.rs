use crate::config::database::DbPool;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        UserRepository { pool }
    }

    pub fn get_users(&self) -> Result<Vec<User>, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        users.load::<User>(&mut conn)
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut conn)
    }

    pub fn get_user(&self, user_id: Uuid) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        users.find(user_id).get_result::<User>(&mut conn)
    }

    pub fn update_user(&self, user_id: Uuid, user_upd: User) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(users.find(user_id))
            .set(&user_upd)
            .get_result::<User>(&mut conn)
    }

    pub fn delete_user(&self, user_id: Uuid) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::delete(users.find(user_id)).get_result(&mut conn)
    }
}
