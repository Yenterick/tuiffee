use ratatui::widgets::ListState;

pub struct PreparationMethodState {
    pub list_state: ListState,
}

impl PreparationMethodState {
    pub fn new() -> Self {
        let mut status: PreparationMethodState = Self {
            list_state: ListState::default(),
        };

        status.list_state.select_first();
        status
    }
}
