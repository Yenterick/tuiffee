use crate::presentation::events::pages::Pages;

pub enum ScreenAction {
    None,
    Confirm,
    Back,
    Exit,
    GoTo(Pages),
    NextPage,
    PreviousPage,
    LoadId(i32),
}
