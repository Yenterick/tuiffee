use crate::{config::db::DbError, config::schema::method, models::method::Method};
use diesel::prelude::*;

pub struct MethodRepository;

impl MethodRepository {
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Method>, DbError> {
        let methods = method::table.load(conn)?;
        Ok(methods)
    }
}
