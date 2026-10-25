use diesel::prelude::*;
use std::{env, fmt};

#[derive(Debug)]
pub enum DbError {
    ConnectionError(diesel::ConnectionError),
    QueryError(diesel::result::Error),
    NotFound,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionError(err) => write!(f, "Database connection error: {}", err),
            DbError::QueryError(err) => write!(f, "Database query error: {}", err),
            DbError::NotFound => write!(f, "Record not found"),
        }
    }
}

impl std::error::Error for DbError {}

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
