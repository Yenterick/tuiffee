use crate::{
    models::{
        blooming::Blooming, coffee::Coffee, pouring::Pouring,
        preparation_technique::PreparationTechnique,
    },
    presentation::events::pages::Pages,
};

pub enum ScreenAction {
    None,
    Confirm,
    Back,
    Exit,
    GoTo(Pages),
    NextPage,
    PreviousPage,
    Error(String),
    LoadId(i32),
    LoadVarieties,
    SaveCoffee(Coffee),
    SavePreparationTechnique(PreparationTechnique, Vec<Blooming>, Vec<Pouring>),
}
