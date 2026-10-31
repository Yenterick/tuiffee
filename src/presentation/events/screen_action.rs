use crate::{models::coffee::Coffee, presentation::events::pages::Pages};

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
}
