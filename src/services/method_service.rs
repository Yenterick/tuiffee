use crate::{
    config::db::DbError, models::method::Method, repositories::method_repository::MethodRepository,
};
use diesel::prelude::*;

pub struct MethodService;

impl MethodService {
    pub fn list_methods(conn: &mut SqliteConnection) -> Result<Vec<Method>, DbError> {
        let methods = MethodRepository::get_all(conn)?;
        Ok(methods)
    }
}
