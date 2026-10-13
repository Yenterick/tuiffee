use diesel::prelude::*;
use std::env;

#[derive(Debug)]
pub enum DbError {
    ConnectionError(diesel::ConnectionError),
    QueryError(diesel::result::Error),
    NotFound,
}

impl From<diesel::ConnectionError> for DbError {
    fn from(err: diesel::ConnectionError) -> Self {
        DbError::ConnectionError(err)
    }
}

impl From<diesel::result::Error> for DbError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => DbError::NotFound,
            _ => DbError::QueryError(err),
        }
    }
}

pub fn establish_connection() -> Result<SqliteConnection, DbError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let conn = SqliteConnection::establish(&database_url)?;

    Ok(conn)
}
