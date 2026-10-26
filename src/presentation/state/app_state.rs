use crate::presentation::{
    components::confirmation_modal::ConfirmationModal,
    events::pages::Pages,
    state::{
        add_coffee_variety_modal_state::AddCoffeeVarietyModalState,
        coffee_varieties_state::CoffeeVarietiesState, home_state::HomeState,
    },
};

pub struct AppState {
    pub current_page: Pages,
    pub confirmation_modal: Option<ConfirmationModal>,
    pub home_state: HomeState,
    pub coffee_varieties_state: CoffeeVarietiesState,
    pub add_coffee_varieties_modal_state: Option<AddCoffeeVarietyModalState>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            current_page: Pages::Home,
            confirmation_modal: None,
            home_state: HomeState::new(),
            coffee_varieties_state: CoffeeVarietiesState::new(),
            add_coffee_varieties_modal_state: None,
        }
    }
}
