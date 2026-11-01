use diesel::prelude::*;

use crate::{
    config::db::DbError, models::pouring::Pouring,
    repositories::pouring_repository::PouringRepository,
};

pub struct PouringService;

impl PouringService {
    pub fn create(conn: &mut SqliteConnection, pouring: &Pouring) -> Result<usize, DbError> {
        let id = PouringRepository::insert(conn, pouring)?;
        Ok(id)
    }
}
