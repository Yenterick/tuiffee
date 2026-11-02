use ratatui::widgets::ListState;

use crate::presentation::{
    components::add_preparation_technique_modal::AddPreparationTechniqueModal,
    state::add_preparation_technique_modal_state::AddPreparationTechniqueModalState,
};

pub struct PreparationMethodState {
    pub list_state: ListState,
    pub add_preparation_technique_modal_state: Option<AddPreparationTechniqueModalState>,
    pub add_preparation_technique_modal: Option<AddPreparationTechniqueModal>,
}

impl PreparationMethodState {
    pub fn new() -> Self {
        let mut status: PreparationMethodState = Self {
            list_state: ListState::default(),
            add_preparation_technique_modal_state: None,
            add_preparation_technique_modal: None,
        };

        status.list_state.select_first();
        status
    }
}
