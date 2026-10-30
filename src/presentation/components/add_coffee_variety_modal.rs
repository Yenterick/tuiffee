use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
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
        match (key.code, key.modifiers) {
            (KeyCode::Esc, _) => ScreenAction::Back,

            (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                if state.list_state.selected().unwrap_or(0) < 6 {
                    state.list_state.select_next();
                }
                ScreenAction::None
            }

            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                if state.list_state.selected().unwrap_or(0) > 0 {
                    state.list_state.select_previous();
                }
                ScreenAction::None
            }

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Length(25));

        Clear.render(centered_area, buf);

        let [
            _,
            brand_area,
            name_area,
            descriptive_notes_area,
            location_area,
            roast_level_area,
            process_type_area,
            variety_list_area,
            _,
        ] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .margin(1)
        .areas(centered_area);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Add a New Coffee ")
                    .bold()
                    .alignment(Alignment::Right),
            )
            .title_bottom(Line::from_iter([
                Span::from(" esc "),
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

        let brand_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Brand ").alignment(Alignment::Right).style(
                    Style::default()
                        .bg(if state.list_state.selected() == Some(0) {
                            ColorScheme::CHICORY_COFFEE
                        } else {
                            ColorScheme::DARK_IRIS
                        })
                        .fg(if state.list_state.selected() == Some(0) {
                            ColorScheme::DARK_IRIS
                        } else {
                            ColorScheme::CHICORY_COFFEE
                        })
                        .bold(),
                ),
            );

        Line::from(state.brand_buffer.clone()).render(brand_area.inner(Margin::from(0)), buf);

        brand_block.render(brand_area, buf);

        let name_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Name ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(1) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(1) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        name_block.render(name_area, buf);

        let descriptive_notes_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Descriptive Notes ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(2) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(2) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        descriptive_notes_block.render(descriptive_notes_area, buf);

        let location_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Location ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(3) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(3) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        location_block.render(location_area, buf);

        let roast_level_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Roast Level ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(4) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(4) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        roast_level_block.render(roast_level_area, buf);

        let proccess_type_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Process Type ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(5) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(5) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        proccess_type_block.render(process_type_area, buf);

        let variety_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Variety ")
                    .bold()
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(6) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(6) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        variety_block.render(variety_list_area, buf)
    }
}
