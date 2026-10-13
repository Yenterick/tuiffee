use crate::config::db::DbError;
use crate::config::schema::variety;
use crate::models::variety::Variety;
use diesel::prelude::*;

pub struct VarietyRepository;

impl VarietyRepository {
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Variety>, DbError> {
        let varieties = variety::table.load(conn)?;
        Ok(varieties)
    }
}
