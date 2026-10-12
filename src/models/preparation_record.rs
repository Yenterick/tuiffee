use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::config::schema::preparation_record)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(crate::models::cupping::Cupping))]
pub struct PreparationRecord {
    pub id: Option<i32>,
    pub method_id: i32,
    pub coffee_id: i32,
    pub preparation_technique_id: i32,
    created_at: Option<String>,
    updated_at: Option<String>,
}
