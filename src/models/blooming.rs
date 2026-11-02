use diesel::prelude::*;

use crate::config::schema::blooming::preparation_technique_id;

#[derive(Queryable, Selectable, Identifiable, Insertable)]
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

impl Blooming {
    pub fn from(water_g: f32, time_s: i32, sequency: i32) -> Self {
        Self {
            id: None,
            water_g,
            time_s,
            sequency,
            preparation_technique_id: -1,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn assign(&mut self, id: i32) {
        self.preparation_technique_id = id;
    }
}
