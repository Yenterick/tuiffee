use diesel::prelude::*;

use crate::{
    config::db::DbError,
    models::preparation_technique::{
        FullPreparationTechnique, PreparationTechnique, PreparationTechniqueSummary,
    },
    repositories::preparation_technique_repository::PreparationTechniqueRepository,
};

pub struct PreparationTechniqueService;

impl PreparationTechniqueService {
    pub fn list(
        conn: &mut SqliteConnection,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<PreparationTechniqueSummary>, DbError> {
        let preparation_techniques =
            PreparationTechniqueRepository::get_all(conn, page, page_size)?;
        Ok(preparation_techniques)
    }

    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<FullPreparationTechnique, DbError> {
        let preparation_technique = PreparationTechniqueRepository::get_by_id(conn, id)?;
        Ok(preparation_technique)
    }

    pub fn create(
        conn: &mut SqliteConnection,
        preparation_technique: &PreparationTechnique,
    ) -> Result<i32, DbError> {
        let id = PreparationTechniqueRepository::insert(conn, preparation_technique)?;
        Ok(id)
    }

    pub fn remove(conn: &mut SqliteConnection, id: i32) -> Result<usize, DbError> {
        let affected_rows = PreparationTechniqueRepository::delete(conn, id)?;
        Ok(affected_rows)
    }
}
