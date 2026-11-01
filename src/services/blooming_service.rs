use diesel::prelude::*;

use crate::{
    config::db::DbError, models::blooming::Blooming,
    repositories::blooming_repository::BloomingRepostory,
};

pub struct BloomingService;

impl BloomingService {
    pub fn create(conn: &mut SqliteConnection, blooming: &Blooming) -> Result<usize, DbError> {
        let id = BloomingRepostory::insert(conn, blooming)?;
        Ok(id)
    }
}
