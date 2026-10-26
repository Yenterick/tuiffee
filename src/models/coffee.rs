use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::config::schema::coffee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Coffee {
    pub id: Option<i32>,
    pub brand: String,
    pub name: Option<String>,
    pub roasting_level: String,
    pub process_type: String,
    pub descriptive_notes: Option<String>,
    pub location: Option<String>,
    pub variety_id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Queryable)]
pub struct CoffeeSummary {
    pub id: Option<i32>,
    pub brand: String,
    pub name: Option<String>,
    pub process_type: String,
    pub variety_name: String,
}
