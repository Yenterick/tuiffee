use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Widget},
};

use crate::{
    models::variety::Variety,
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::add_coffee_variety_modal_state::AddCoffeeVarietyModalState,
        theme::color_scheme::ColorScheme,
    },
};

pub struct AddCoffeeVarietyModal {
    pub varieties: Vec<Variety>,
}

impl AddCoffeeVarietyModal {
    pub fn new() -> Self {
        Self {
            varieties: Vec::new(),
        }
    }
}

impl Page for AddCoffeeVarietyModal {
    type State = AddCoffeeVarietyModalState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match key.code {
            KeyCode::Char('q') => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Percentage(50));

        Clear.render(centered_area, buf);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from("Add a New Coffee ")
                    .bold()
                    .alignment(Alignment::Right),
            )
            .title_bottom(Line::from_iter([
                Span::from(" q "),
                Span::from("back ")
                    .bold()
                    .fg(ColorScheme::DARK_IRIS)
                    .bg(ColorScheme::CHICORY_COFFEE),
                Span::from(" ^k "),
                Span::from("up ")
                    .bold()
                    .fg(ColorScheme::DARK_IRIS)
                    .bg(ColorScheme::CHICORY_COFFEE),
                Span::from(" ^j "),
                Span::from("down ")
                    .bold()
                    .fg(ColorScheme::DARK_IRIS)
                    .bg(ColorScheme::CHICORY_COFFEE),
                Span::from(" ↵ "),
                Span::from("confirm ")
                    .bold()
                    .fg(ColorScheme::DARK_IRIS)
                    .bg(ColorScheme::CHICORY_COFFEE),
            ]))
            .render(centered_area, buf);
    }
}
