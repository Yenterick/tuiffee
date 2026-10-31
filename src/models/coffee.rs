use diesel::prelude::*;

use crate::models::{process_type::ProcessType, roasting_level::RoastingLevel};

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

impl Coffee {
    pub fn from(
        brand: String,
        name: Option<String>,
        roasting_level: RoastingLevel,
        process_type: ProcessType,
        descriptive_notes: Option<String>,
        location: Option<String>,
        variety_id: i32,
    ) -> Coffee {
        Coffee {
            id: None,
            brand,
            name,
            roasting_level: roasting_level.to_sql(),
            process_type: process_type.to_sql(),
            descriptive_notes,
            location,
            variety_id,
            created_at: None,
            updated_at: None,
        }
    }
}
