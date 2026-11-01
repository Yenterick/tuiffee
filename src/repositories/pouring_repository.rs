use diesel::prelude::*;

use crate::{
    config::{db::DbError, schema::pouring},
    models::pouring::Pouring,
};

pub struct PouringRepository;

impl PouringRepository {
    pub fn insert(conn: &mut SqliteConnection, pouring: &Pouring) -> Result<usize, DbError> {
        diesel::insert_into(pouring::table)
            .values(pouring)
            .execute(conn)
            .map_err(DbError::from)
    }
}
