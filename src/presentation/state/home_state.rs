use ratatui::widgets::ListState;

pub struct HomeState {
    pub list_state: ListState,
}

impl HomeState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
        };

        status.list_state.select_first();
        status
    }
}
