use diesel::prelude::*;

use crate::models::{blooming::Blooming, grind_level::GrindLevel, pouring::Pouring};

#[derive(Queryable, Selectable, Insertable, Identifiable)]
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

#[derive(Queryable)]
pub struct PreparationTechniqueSummary {
    pub id: Option<i32>,
    pub name: String,
    pub people_target: i32,
}

pub struct FullPreparationTechnique {
    pub id: Option<i32>,
    pub name: String,
    pub grind_level: String,
    pub ratio: i32,
    pub people_target: i32,
    pub coffee_g: f32,
    pub pourings: Vec<Pouring>,
    pub bloomings: Vec<Blooming>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl PreparationTechnique {
    pub fn from(
        name: String,
        grind_level: GrindLevel,
        ratio: i32,
        people_target: i32,
        coffee_g: f32,
    ) -> Self {
        Self {
            id: None,
            name,
            grind_level: grind_level.to_sql(),
            ratio,
            people_target,
            coffee_g,
            created_at: None,
            updated_at: None,
        }
    }
}
