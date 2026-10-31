use crate::{
    config::db::DbError, models::variety::Variety,
    repositories::varieties_repository::VarietyRepository,
};
use diesel::prelude::*;

pub struct VarietyService;

impl VarietyService {
    pub fn list_varieties(conn: &mut SqliteConnection) -> Result<Vec<Variety>, DbError> {
        let varieties = VarietyRepository::get_all(conn)?;
        Ok(varieties)
    }

    pub fn count_varieties(conn: &mut SqliteConnection) -> Result<i64, DbError> {
        let total_varieties = VarietyRepository::count(conn)?;
        Ok(total_varieties)
    }
}
