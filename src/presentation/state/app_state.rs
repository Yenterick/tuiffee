use crate::presentation::components::confirmation_modal::ConfirmationModal;

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
        }
    }
}
