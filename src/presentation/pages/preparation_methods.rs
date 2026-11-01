use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    models::preparation_technique::PreparationTechnique,
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::preparation_methods_state::PreparationMethodState,
    },
};

pub struct PreparationMethods {
    pub current_item: Option<PreparationTechnique>,
    pub current_items: Vec<Pepara>,
}

impl PreparationMethods {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for PreparationMethods {
    type State = PreparationMethodState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match key.code {
            KeyCode::Char('q') => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [list_area, description_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(area);

        Block::new().borders(Borders::RIGHT).render(list_area, buf);

        let [inner_list, bottom_title, add_button, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(list_area);
    }
}
