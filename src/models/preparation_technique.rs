use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::config::schema::preparation_technique)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(crate::models::preparation_record::PreparationRecord))]
pub struct PreparationTechnique {
    pub id: Option<i32>,
    pub name: String,
    pub grind_level: String,
    pub ratio: i32,
    pub people_target: i32,
    pub coffee_g: f32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
