use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::config::schema::method)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(PreparationRecord))]
pub struct Method {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
