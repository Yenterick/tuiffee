use ratatui::widgets::ListState;

pub struct AddCoffeeVarietyModalState {
    pub list_state: ListState,
    pub brand_buffer: String,
    pub name_buffer: String,
    pub descriptive_notes_buffer: String,
    pub location_buffer: String,
    pub roast_level_list_state: ListState,
    pub process_type_list_state: ListState,
    pub variety_list_state: ListState,
}

impl AddCoffeeVarietyModalState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
            brand_buffer: String::new(),
            name_buffer: String::new(),
            descriptive_notes_buffer: String::new(),
            location_buffer: String::new(),
            roast_level_list_state: ListState::default(),
            process_type_list_state: ListState::default(),
            variety_list_state: ListState::default(),
        };

        status.list_state.select_first();
        status.roast_level_list_state.select_first();
        status.process_type_list_state.select_first();
        status.variety_list_state.select_first();

        status
    }
}
