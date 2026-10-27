use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
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
                Line::from_iter([
                    Span::from(" ^k "),
                    Span::from("Up ")
                        .bold()
                        .fg(ColorScheme::CHICORY_COFFEE)
                        .bg(ColorScheme::DARK_IRIS),
                ])
                .centered(),
            )
            .title_bottom(
                Line::from_iter([
                    Span::from(" ^j "),
                    Span::from("Down ")
                        .bold()
                        .fg(ColorScheme::CHICORY_COFFEE)
                        .bg(ColorScheme::DARK_IRIS),
                ])
                .centered(),
            )
            .render(centered_area, buf);
    }
}
