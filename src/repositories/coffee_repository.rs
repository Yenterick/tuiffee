use crate::{
    config::db::DbError,
    config::schema::{coffee, variety},
    models::coffee::{Coffee, CoffeeSummary},
};
use diesel::prelude::*;

pub struct CoffeeRepository;

impl CoffeeRepository {
    pub fn get_all(
        conn: &mut SqliteConnection,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<CoffeeSummary>, DbError> {
        let records_to_skip = (page - 1) * page_size;

        coffee::table
            .limit(page_size as i64)
            .offset(records_to_skip as i64)
            .inner_join(variety::table)
            .select((
                coffee::brand,
                coffee::name,
                coffee::process_type,
                variety::name,
            ))
            .load::<CoffeeSummary>(conn)
            .map_err(DbError::from)
    }

    pub fn get_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Coffee, DbError> {
        coffee::table
            .filter(coffee::id.eq(id))
            .first::<Coffee>(conn)
            .map_err(DbError::from)
    }

    pub fn insert(conn: &mut SqliteConnection, coffee: &Coffee) -> Result<usize, DbError> {
        diesel::insert_into(coffee::table)
            .values(coffee)
            .execute(conn)
            .map_err(DbError::from)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, DbError> {
        diesel::delete(coffee::table)
            .filter(coffee::id.eq(id))
            .execute(conn)
            .map_err(DbError::from)
    }
}
