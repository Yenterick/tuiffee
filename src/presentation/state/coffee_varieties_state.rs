use ratatui::widgets::ListState;

pub struct CoffeeVarietiesState {
    pub list_state: ListState,
    pub selected_option: Option<usize>,
}

impl CoffeeVarietiesState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
            selected_option: None,
        };

        status.list_state.select_first();
        status
    }
}
