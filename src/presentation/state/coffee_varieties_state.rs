use ratatui::widgets::ListState;

pub struct CoffeeVarietiesState {
    pub list_state: ListState,
}

impl CoffeeVarietiesState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
        };

        status.list_state.select_first();
        status
    }
}
