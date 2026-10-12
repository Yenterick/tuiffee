use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::config::schema::blooming)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(crate::models::preparation_technique::PreparationTechnique))]
pub struct Blooming {
    pub id: Option<i32>,
    pub water_g: f32,
    pub time_s: i32,
    pub sequency: i32,
    pub preparation_technique_id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
