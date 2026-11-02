use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Clear, Widget},
};

use crate::{
    models::{blooming::Blooming, pouring::Pouring},
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::add_preparation_technique_modal_state::AddPreparationTechniqueModalState,
    },
};

pub struct AddPreparationTechniqueModal {
    pub bloomings: Vec<Blooming>,
    pub pourings: Vec<Pouring>,
}

impl AddPreparationTechniqueModal {
    pub fn new() -> Self {
        Self {
            bloomings: Vec::new(),
            pourings: Vec::new(),
        }
    }
}

impl Page for AddPreparationTechniqueModal {
    type State = AddPreparationTechniqueModalState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match (key.code, key.modifiers) {
            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Length(25));

        Clear.render(centered_area, buf);

        let [
            _,
            name_area,
            grind_level_area,
            ratio_area,
            people_target_area,
            _,
        ] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .margin(1)
        .areas(centered_area);
    }
}
