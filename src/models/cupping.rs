use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::config::schema::cupping)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Cupping {
    pub id: Option<i32>,
    pub author: String,
    pub aroma: Option<i32>,
    pub flavor: Option<i32>,
    pub acidity: Option<i32>,
    pub sweetness: Option<i32>,
    pub body: Option<i32>,
    pub aftertaste: Option<i32>,
    pub balance: Option<i32>,
    pub observations: Option<String>,
    pub preparation_record_id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
