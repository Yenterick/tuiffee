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
