use crate::{
    config::db::DbError,
    config::schema::{coffee, method, preparation_record, preparation_technique},
    models::preparation_record::{PreparationRecord, PreparationRecordSummary},
};
use diesel::prelude::*;

pub struct PreparationRecordRepository;

impl PreparationRecordRepository {
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<PreparationRecordSummary>, DbError> {
        preparation_record::table
            .inner_join(coffee::table)
            .inner_join(method::table)
            .inner_join(preparation_technique::table)
            .select((
                preparation_record::id,
                coffee::brand,
                coffee::name,
                preparation_technique::name,
                preparation_record::created_at,
            ))
            .load::<PreparationRecordSummary>(conn)
            .map_err(DbError::from)
    }

    pub fn get_by_id(conn: &mut SqliteConnection, id: i32) -> Result<PreparationRecord, DbError> {
        preparation_record::table
            .filter(preparation_record::id.eq(id))
            .first::<PreparationRecord>(conn)
            .map_err(DbError::from)
    }
}
