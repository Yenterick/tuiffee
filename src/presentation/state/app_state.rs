use crate::presentation::{
    components::confirmation_modal::ConfirmationModal, state::home_state::HomeState,
};

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub home_state: HomeState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            home_state: HomeState::new(),
        }
    }
}
