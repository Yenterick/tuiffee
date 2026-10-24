use crate::presentation::{
    components::confirmation_modal::ConfirmationModal,
    events::pages::Pages,
    state::{coffee_varieties_state::CoffeeVarietiesState, home_state::HomeState},
};

pub struct AppState {
    pub current_page: Pages,
    pub confirmation_modal: Option<ConfirmationModal>,
    pub home_state: HomeState,
    pub coffee_varieties_state: CoffeeVarietiesState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            current_page: Pages::Home,
            confirmation_modal: None,
            home_state: HomeState::new(),
            coffee_varieties_state: CoffeeVarietiesState::new(),
        }
    }
}
