use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::config::schema::variety)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(crate::models::coffee::Coffee))]
pub struct Variety {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
