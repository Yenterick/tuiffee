use crate::{
    config::db::DbError,
    models::coffee::{Coffee, CoffeeSummary},
    repositories::coffee_repository::CoffeeRepository,
};
use diesel::prelude::*;

pub struct CoffeeService;

impl CoffeeService {
    pub fn list(
        conn: &mut SqliteConnection,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<CoffeeSummary>, DbError> {
        let coffee = CoffeeRepository::get_all(conn, page, page_size)?;
        Ok(coffee)
    }

    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<Coffee, DbError> {
        let coffee = CoffeeRepository::get_by_id(conn, id)?;
        Ok(coffee)
    }

    pub fn create(conn: &mut SqliteConnection, coffee: &Coffee) -> Result<usize, DbError> {
        let id = CoffeeRepository::insert(conn, coffee)?;
        Ok(id)
    }

    pub fn remove(conn: &mut SqliteConnection, id: i32) -> Result<usize, DbError> {
        let affected_rows = CoffeeRepository::delete(conn, id)?;
        Ok(affected_rows)
    }
}
