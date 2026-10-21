use crate::{config::db::DbError, config::schema::cupping, models::cupping::Cupping};
use diesel::prelude::*;

pub struct CuppingRepository;

impl CuppingRepository {
    pub fn get_all(
        conn: &mut SqliteConnection,
        preparation_record_id: i32,
    ) -> Result<Vec<Cupping>, DbError> {
        cupping::table
            .filter(cupping::preparation_record_id.eq(preparation_record_id))
            .load::<Cupping>(conn)
            .map_err(DbError::from)
    }

    pub fn get_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Cupping, DbError> {
        cupping::table
            .filter(cupping::id.eq(id))
            .first::<Cupping>(conn)
            .map_err(DbError::from)
    }

    pub fn insert(conn: &mut SqliteConnection, cupping: &Cupping) -> Result<usize, DbError> {
        diesel::insert_into(cupping::table)
            .values(cupping)
            .execute(conn)
            .map_err(DbError::from)
    }
}
