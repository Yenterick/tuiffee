use crate::{config::db::DbError, config::schema::variety, models::variety::Variety};
use diesel::{dsl::count, prelude::*};

pub struct VarietyRepository;

impl VarietyRepository {
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Variety>, DbError> {
        variety::table.load::<Variety>(conn).map_err(DbError::from)
    }

    pub fn count(conn: &mut SqliteConnection) -> Result<i64, DbError> {
        variety::table
            .select(count(variety::id))
            .first(conn)
            .map_err(DbError::from)
    }
}
