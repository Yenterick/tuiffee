use ratatui::widgets::ListState;

pub struct AddPreparationTechniqueModalState {
    pub list_state: ListState,
    pub name_buffer: String,
    pub grind_level_list_state: ListState,
    pub ratio_buffer: String,
    pub people_target_buffer: String,
    pub coffee_g_buffer: String,
    // Blooming
    pub b_water_g_buffer: String,
    pub b_time_s_buffer: String,
    // Pouring
    pub p_water_g_buffer: String,
    pub p_time_s_buffer: String,
}

impl AddPreparationTechniqueModalState {
    pub fn new() -> Self {
        let mut status = Self {
            list_state: ListState::default(),
            name_buffer: String::new(),
            grind_level_list_state: ListState::default(),
            ratio_buffer: String::from("15"),
            people_target_buffer: String::from("1"),
            coffee_g_buffer: String::from("20"),
            b_water_g_buffer: String::from("0.0"),
            b_time_s_buffer: String::from("0"),
            p_water_g_buffer: String::from("0.0"),
            p_time_s_buffer: String::from("0"),
        };

        status.list_state.select_first();
        status.grind_level_list_state.select_first();

        status
    }

    pub fn add_blooming(&mut self) {
        self.b_water_g_buffer = String::from("0.0");
        self.b_time_s_buffer = String::from("0");
    }

    pub fn add_pouring(&mut self) {
        self.p_water_g_buffer = String::from("0.0");
        self.p_time_s_buffer = String::from("0");
    }
}
