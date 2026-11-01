use diesel::prelude::*;

use crate::{
    config::{db::DbError, schema::blooming},
    models::blooming::Blooming,
};

pub struct BloomingRepostory;

impl BloomingRepostory {
    pub fn insert(conn: &mut SqliteConnection, blooming: &Blooming) -> Result<usize, DbError> {
        diesel::insert_into(blooming::table)
            .values(blooming)
            .execute(conn)
            .map_err(DbError::from)
    }
}
