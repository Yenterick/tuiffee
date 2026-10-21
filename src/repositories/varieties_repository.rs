use crate::{config::db::DbError, config::schema::variety, models::variety::Variety};
use diesel::prelude::*;

pub struct VarietyRepository;

impl VarietyRepository {
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Variety>, DbError> {
        variety::table.load::<Variety>(conn).map_err(DbError::from)
    }
}
