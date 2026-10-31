use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Widget},
};

use crate::{
    models::{
        coffee::Coffee, process_type::ProcessType, roasting_level::RoastingLevel, variety::Variety,
    },
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::add_coffee_variety_modal_state::AddCoffeeVarietyModalState,
        theme::color_scheme::ColorScheme,
    },
};

pub struct AddCoffeeVarietyModal {
    pub varieties: Vec<Variety>,
    pub total_varieties: i64,
}

impl AddCoffeeVarietyModal {
    pub fn new() -> Self {
        Self {
            varieties: Vec::new(),
            total_varieties: 0,
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

            (KeyCode::Char(char), (keycode)) => match state.list_state.selected().unwrap_or(0) {
                0 => {
                    if state.brand_buffer.len() < 64 {
                        state.brand_buffer.push(char);
                    }
                    ScreenAction::None
                }

                1 => {
                    if state.name_buffer.len() < 64 {
                        state.name_buffer.push(char);
                    }
                    ScreenAction::None
                }

                2 => {
                    if state.descriptive_notes_buffer.len() < 64 {
                        state.descriptive_notes_buffer.push(char);
                    }
                    ScreenAction::None
                }

                3 => {
                    if state.location_buffer.len() < 64 {
                        state.location_buffer.push(char);
                    }
                    ScreenAction::None
                }

                4 => {
                    if char == 'l' && keycode == KeyModifiers::CONTROL {
                        if state.roast_level_list_state.selected().unwrap_or(0) < 4 - 1 {
                            state.roast_level_list_state.select_next();
                        }
                    } else if char == 'h' && keycode == KeyModifiers::CONTROL {
                        if state.roast_level_list_state.selected().unwrap_or(0) > 0 {
                            state.roast_level_list_state.select_previous();
                        }
                    }
                    ScreenAction::None
                }

                5 => {
                    if char == 'l' && keycode == KeyModifiers::CONTROL {
                        if state.process_type_list_state.selected().unwrap_or(0) < 4 - 1 {
                            state.process_type_list_state.select_next();
                        }
                    } else if char == 'h' && keycode == KeyModifiers::CONTROL {
                        if state.process_type_list_state.selected().unwrap_or(0) > 0 {
                            state.process_type_list_state.select_previous();
                        }
                    }
                    ScreenAction::None
                }

                6 => {
                    if char == 'l' && keycode == KeyModifiers::CONTROL {
                        if state.variety_list_state.selected().unwrap_or(0)
                            < self.total_varieties as usize - 1
                        {
                            state.variety_list_state.select_next();
                        }
                    } else if char == 'h' && keycode == KeyModifiers::CONTROL {
                        if state.variety_list_state.selected().unwrap_or(0) > 0 {
                            state.variety_list_state.select_previous();
                        }
                    }
                    ScreenAction::None
                }

                _ => ScreenAction::None,
            },

            (KeyCode::Backspace, _) => match state.list_state.selected().unwrap_or(0) {
                0 => {
                    state.brand_buffer.pop();
                    ScreenAction::None
                }

                1 => {
                    state.name_buffer.pop();
                    ScreenAction::None
                }

                2 => {
                    state.descriptive_notes_buffer.pop();
                    ScreenAction::None
                }

                3 => {
                    state.location_buffer.pop();
                    ScreenAction::None
                }

                _ => ScreenAction::None,
            },

            (KeyCode::Enter, _) => {
                if !state.brand_buffer.is_empty() {
                    ScreenAction::SaveCoffee(Coffee::from(
                        state.brand_buffer.clone(),
                        if state.name_buffer.is_empty() {
                            None
                        } else {
                            Some(state.name_buffer.clone())
                        },
                        match state.roast_level_list_state.selected().unwrap_or(0) {
                            0 => RoastingLevel::Light,
                            1 => RoastingLevel::Medium,
                            2 => RoastingLevel::MediumDark,
                            3 => RoastingLevel::Dark,

                            _ => RoastingLevel::Medium,
                        },
                        match state.process_type_list_state.selected().unwrap_or(0) {
                            0 => ProcessType::Washed,
                            1 => ProcessType::Honey,
                            2 => ProcessType::Natural,
                            3 => ProcessType::Culturing,

                            _ => ProcessType::Washed,
                        },
                        if state.descriptive_notes_buffer.is_empty() {
                            None
                        } else {
                            Some(state.descriptive_notes_buffer.clone())
                        },
                        if state.location_buffer.is_empty() {
                            None
                        } else {
                            Some(state.location_buffer.clone())
                        },
                        self.varieties[state.variety_list_state.selected().unwrap_or(0)]
                            .id
                            .unwrap_or(0),
                    ))
                } else {
                    ScreenAction::Error(String::from("Please fill all the required fields!"))
                }
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
                Span::from(" ^h "),
                Span::from("left ")
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
                Span::from(" ^l "),
                Span::from("right ")
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

        let brand_inner_block = brand_block.inner(brand_area);
        brand_block.render(brand_area, buf);
        Line::from(state.brand_buffer.clone()).render(brand_inner_block, buf);

        let name_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Name <Optional> ")
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

        let name_inner_block = name_block.inner(name_area);
        name_block.render(name_area, buf);
        Line::from(state.name_buffer.clone()).render(name_inner_block, buf);

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

        let descriptive_notes_inner_block = descriptive_notes_block.inner(descriptive_notes_area);
        descriptive_notes_block.render(descriptive_notes_area, buf);
        Line::from(state.descriptive_notes_buffer.clone())
            .render(descriptive_notes_inner_block, buf);

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

        let location_inner_block = location_block.inner(location_area);
        location_block.render(location_area, buf);
        Line::from(state.location_buffer.clone()).render(location_inner_block, buf);

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

        let roast_level_inner_block = roast_level_block.inner(roast_level_area);
        roast_level_block.render(roast_level_area, buf);
        Line::from(match state.roast_level_list_state.selected().unwrap_or(0) {
            0 => String::from(" Light > "),
            1 => String::from(" < Medium > "),
            2 => String::from(" < Medium-Dark > "),
            3 => String::from(" < Dark "),
            _ => String::from(" Unavailable! "),
        })
        .render(roast_level_inner_block, buf);

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

        let process_type_inner_block = proccess_type_block.inner(process_type_area);
        proccess_type_block.render(process_type_area, buf);
        Line::from(
            match state.process_type_list_state.selected().unwrap_or(0) {
                0 => String::from(" Washed > "),
                1 => String::from(" < Honey > "),
                2 => String::from(" < Natural > "),
                3 => String::from(" < Culturing "),
                _ => String::from(" Unavailable! "),
            },
        )
        .render(process_type_inner_block, buf);

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

        let variety_inner_block = variety_block.inner(variety_list_area);
        variety_block.render(variety_list_area, buf);
        Line::from_iter([
            Span::from(if state.variety_list_state.selected().unwrap_or(0) > 0 {
                String::from(" < ")
            } else {
                String::from(" ")
            }),
            Span::from(
                self.varieties[state.variety_list_state.selected().unwrap_or(0)]
                    .name
                    .clone(),
            ),
            Span::from(
                if state.variety_list_state.selected().unwrap_or(0)
                    < self.total_varieties as usize - 1
                {
                    String::from(" > ")
                } else {
                    String::from(" ")
                },
            ),
        ])
        .render(variety_inner_block, buf);
    }
}
