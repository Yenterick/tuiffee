use ratatui::widgets::ListState;

use crate::presentation::{
    components::add_coffee_variety_modal::AddCoffeeVarietyModal,
    state::add_coffee_variety_modal_state::AddCoffeeVarietyModalState,
};

pub struct CoffeeVarietiesState {
    pub list_state: ListState,
    pub add_coffee_variety_modal: Option<AddCoffeeVarietyModal>,
    pub add_coffee_variety_modal_state: Option<AddCoffeeVarietyModalState>,
}

impl CoffeeVarietiesState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
            add_coffee_variety_modal: None,
            add_coffee_variety_modal_state: None,
        };

        status.list_state.select_first();
        status
    }
}
