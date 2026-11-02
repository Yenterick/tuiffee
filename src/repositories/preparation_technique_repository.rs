use diesel::prelude::*;

use crate::{
    config::{
        db::DbError,
        schema::{blooming, pouring, preparation_technique},
    },
    models::{
        blooming::Blooming,
        pouring::Pouring,
        preparation_technique::{
            FullPreparationTechnique, PreparationTechnique, PreparationTechniqueSummary,
        },
    },
};

pub struct PreparationTechniqueRepository;

impl PreparationTechniqueRepository {
    pub fn get_all(
        conn: &mut SqliteConnection,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<PreparationTechniqueSummary>, DbError> {
        let records_to_skip = (page - 1) * page_size;

        preparation_technique::table
            .limit(page_size as i64)
            .offset(records_to_skip as i64)
            .select((
                preparation_technique::id,
                preparation_technique::name,
                preparation_technique::people_target,
            ))
            .load::<PreparationTechniqueSummary>(conn)
            .map_err(DbError::from)
    }

    pub fn get_by_id(
        conn: &mut SqliteConnection,
        id: i32,
    ) -> Result<FullPreparationTechnique, DbError> {
        let technique = preparation_technique::table
            .filter(preparation_technique::id.eq(id))
            .first::<PreparationTechnique>(conn)?;

        let bloomings = blooming::table
            .filter(blooming::preparation_technique_id.eq(id))
            .order(blooming::sequency.asc())
            .load::<Blooming>(conn)?;
        let pourings = pouring::table
            .filter(pouring::preparation_technique_id.eq(id))
            .order(pouring::sequency.asc())
            .load::<Pouring>(conn)?;

        Ok(FullPreparationTechnique {
            id: technique.id,
            name: technique.name,
            grind_level: technique.grind_level,
            ratio: technique.ratio,
            people_target: technique.people_target,
            coffee_g: technique.coffee_g,
            pourings,
            bloomings,
            created_at: technique.created_at,
            updated_at: technique.updated_at,
        })
    }

    pub fn insert(
        conn: &mut SqliteConnection,
        preparation_technique: &PreparationTechnique,
    ) -> Result<usize, DbError> {
        diesel::insert_into(preparation_technique::table)
            .values(preparation_technique)
            .execute(conn)
            .map_err(DbError::from)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, DbError> {
        diesel::delete(preparation_technique::table)
            .filter(preparation_technique::id.eq(id))
            .execute(conn)
            .map_err(DbError::from)
    }
}
