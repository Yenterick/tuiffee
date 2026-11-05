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
        blooming::Blooming, grind_level::GrindLevel, pouring::Pouring,
        preparation_technique::PreparationTechnique,
    },
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::add_preparation_technique_modal_state::AddPreparationTechniqueModalState,
        theme::color_scheme::ColorScheme,
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
            (KeyCode::Enter, _) => ScreenAction::SavePreparationTechnique(
                PreparationTechnique::from(
                    state.name_buffer.clone(),
                    match state.grind_level_list_state.selected().unwrap_or(0) {
                        0 => GrindLevel::ExtraCoarse,
                        2 => GrindLevel::Coarse,
                        3 => GrindLevel::MediumCoarse,
                        4 => GrindLevel::Medium,
                        5 => GrindLevel::MediumFine,
                        6 => GrindLevel::Fine,
                        7 => GrindLevel::ExtraFine,
                        _ => GrindLevel::Medium,
                    },
                    state
                        .ratio_buffer
                        .parse::<i32>()
                        .expect("Invalid data type for ratio"),
                    state
                        .people_target_buffer
                        .parse::<i32>()
                        .expect("Invalid data type for people target"),
                    state
                        .coffee_g_buffer
                        .parse::<f32>()
                        .expect("Invalid data type for coffee quantity"),
                ),
                self.bloomings.clone(),
                self.pourings.clone(),
            ),

            (KeyCode::Esc, _) => ScreenAction::Back,

            (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                if state.list_state.selected().unwrap_or(0) < 8 {
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
                    if state.name_buffer.len() < 64 {
                        state.name_buffer.push(char);
                    }
                    ScreenAction::None
                }

                1 => {
                    if char == 'l' && keycode == KeyModifiers::CONTROL {
                        if state.grind_level_list_state.selected().unwrap_or(0) < 7 - 1 {
                            state.grind_level_list_state.select_next();
                        }
                    } else if char == 'h' && keycode == KeyModifiers::CONTROL {
                        if state.grind_level_list_state.selected().unwrap_or(0) > 0 {
                            state.grind_level_list_state.select_previous();
                        }
                    }
                    ScreenAction::None
                }

                2 => {
                    if state.ratio_buffer.len() < 64 && is_valid_int_char(char) {
                        state.ratio_buffer.push(char);
                    }
                    ScreenAction::None
                }

                3 => {
                    if state.people_target_buffer.len() < 64 && is_valid_int_char(char) {
                        state.people_target_buffer.push(char);
                    }
                    ScreenAction::None
                }

                4 => {
                    if state.coffee_g_buffer.len() < 64
                        && is_valid_float_char(&state.coffee_g_buffer, char)
                    {
                        state.coffee_g_buffer.push(char);
                    }
                    ScreenAction::None
                }

                5 => {
                    if char == 'c' && keycode == KeyModifiers::CONTROL {
                        self.bloomings.push(Blooming::from(
                            state.b_water_g_buffer.parse::<f32>().unwrap_or(0.0),
                            state.b_time_s_buffer.parse::<i32>().unwrap_or(1),
                            (self.bloomings.len() + 1) as i32,
                        ));
                        state.add_blooming();
                    } else {
                        if state.b_water_g_buffer.len() < 32
                            && is_valid_float_char(&state.b_water_g_buffer, char)
                        {
                            state.b_water_g_buffer.push(char);
                        }
                    }
                    ScreenAction::None
                }

                6 => {
                    if char == 'c' && keycode == KeyModifiers::CONTROL {
                        self.bloomings.push(Blooming::from(
                            state.b_water_g_buffer.parse::<f32>().unwrap_or(0.0),
                            state.b_time_s_buffer.parse::<i32>().unwrap_or(1),
                            (self.bloomings.len() + 1) as i32,
                        ));
                        state.add_blooming();
                    } else {
                        if state.b_time_s_buffer.len() < 32 && is_valid_int_char(char) {
                            state.b_time_s_buffer.push(char);
                        }
                    }
                    ScreenAction::None
                }

                7 => {
                    if char == 'c' && keycode == KeyModifiers::CONTROL {
                        self.pourings.push(Pouring::from(
                            state.b_water_g_buffer.parse::<f32>().unwrap_or(0.0),
                            state.b_time_s_buffer.parse::<i32>().unwrap_or(1),
                            (self.bloomings.len() + 1) as i32,
                        ));
                        state.add_pouring();
                    } else {
                        if state.p_water_g_buffer.len() < 32
                            && is_valid_float_char(&state.p_water_g_buffer, char)
                        {
                            state.p_water_g_buffer.push(char);
                        }
                    }
                    ScreenAction::None
                }

                8 => {
                    if char == 'c' && keycode == KeyModifiers::CONTROL {
                        self.pourings.push(Pouring::from(
                            state.b_water_g_buffer.parse::<f32>().unwrap_or(0.0),
                            state.b_time_s_buffer.parse::<i32>().unwrap_or(1),
                            (self.bloomings.len() + 1) as i32,
                        ));
                        state.add_pouring();
                    } else {
                        if state.p_time_s_buffer.len() < 32 && is_valid_int_char(char) {
                            state.p_time_s_buffer.push(char);
                        }
                    }
                    ScreenAction::None
                }

                _ => ScreenAction::None,
            },

            (KeyCode::Backspace, _) => match state.list_state.selected().unwrap_or(0) {
                0 => {
                    state.name_buffer.pop();
                    ScreenAction::None
                }

                2 => {
                    state.ratio_buffer.pop();
                    ScreenAction::None
                }

                3 => {
                    state.people_target_buffer.pop();
                    ScreenAction::None
                }

                4 => {
                    state.coffee_g_buffer.pop();
                    ScreenAction::None
                }

                5 => {
                    state.b_water_g_buffer.pop();
                    ScreenAction::None
                }

                6 => {
                    state.b_time_s_buffer.pop();
                    ScreenAction::None
                }

                7 => {
                    state.p_water_g_buffer.pop();
                    ScreenAction::None
                }

                8 => {
                    state.p_time_s_buffer.pop();
                    ScreenAction::None
                }

                _ => ScreenAction::None,
            },

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Length(25));
        let [inputs_area, lists_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(centered_area);

        let [blooming_list_area, _, pouring_list_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .margin(2)
        .areas(lists_area);

        Clear.render(centered_area, buf);

        let [
            _,
            name_area,
            grind_level_area,
            ratio_area,
            people_target_area,
            coffee_g_area,
            blooming_area,
            pouring_area,
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
        .areas(inputs_area);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Add a New Preparation Technique ")
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

        let name_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Name ").alignment(Alignment::Right).style(
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

        let name_inner_block = name_block.inner(name_area);
        name_block.render(name_area, buf);
        Line::from(state.name_buffer.clone()).render(name_inner_block, buf);

        let grind_level_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Grind Level ")
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

        let roast_level_inner_block = grind_level_block.inner(grind_level_area);
        grind_level_block.render(grind_level_area, buf);
        Line::from(match state.grind_level_list_state.selected().unwrap_or(0) {
            0 => String::from(" Extra-coarse > "),
            1 => String::from(" < Coarse > "),
            2 => String::from(" < Medium-coarse > "),
            3 => String::from(" < Medium > "),
            4 => String::from(" < Medium-fine > "),
            5 => String::from(" < Fine > "),
            6 => String::from(" < Extra-fine "),
            _ => String::from(" Unavailable! "),
        })
        .render(roast_level_inner_block, buf);

        let ratio_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Ratio ").alignment(Alignment::Right).style(
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

        let ratio_inner_block = ratio_block.inner(ratio_area);
        ratio_block.render(ratio_area, buf);

        let [ratio_unit_area, ratio_text_area] =
            Layout::horizontal([Constraint::Length(4), Constraint::Fill(1)])
                .areas(ratio_inner_block);

        Line::from(" 1: ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(ratio_unit_area, buf);
        Line::from(state.ratio_buffer.clone()).render(ratio_text_area, buf);

        let people_target_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" People Target ")
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

        let people_target_inner_block = people_target_block.inner(people_target_area);
        people_target_block.render(people_target_area, buf);

        let [people_target_text_area, people_target_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(8)])
                .areas(people_target_inner_block);

        Line::from(state.people_target_buffer.clone()).render(people_target_text_area, buf);
        Line::from(" people ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(people_target_unit_area, buf);

        let coffee_g_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Coffee ").alignment(Alignment::Right).style(
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

        let coffee_g_inner_block = coffee_g_block.inner(coffee_g_area);
        coffee_g_block.render(coffee_g_area, buf);

        let [coffee_g_text_area, coffee_g_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(3)])
                .areas(coffee_g_inner_block);

        Line::from(state.coffee_g_buffer.clone()).render(coffee_g_text_area, buf);
        Line::from(" g ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(coffee_g_unit_area, buf);

        let [b_water_area, _, b_time_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(blooming_area);
        let [p_water_area, _, p_time_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(pouring_area);

        let b_water_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Blooming Water ")
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

        let b_time_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Blooming Time ")
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

        let p_water_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Pouring Water ")
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(7) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(7) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        let p_time_block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::DARK_IRIS)
            .fg(ColorScheme::CHICORY_COFFEE)
            .title(
                Line::from(" Pouring Time ")
                    .alignment(Alignment::Right)
                    .style(
                        Style::default()
                            .bg(if state.list_state.selected() == Some(8) {
                                ColorScheme::CHICORY_COFFEE
                            } else {
                                ColorScheme::DARK_IRIS
                            })
                            .fg(if state.list_state.selected() == Some(8) {
                                ColorScheme::DARK_IRIS
                            } else {
                                ColorScheme::CHICORY_COFFEE
                            })
                            .bold(),
                    ),
            );

        let b_water_inner_block = b_water_block.inner(b_water_area);
        let b_time_inner_block = b_time_block.inner(b_time_area);

        b_water_block.render(b_water_area, buf);
        b_time_block.render(b_time_area, buf);

        let [b_water_text_area, b_water_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(3)])
                .areas(b_water_inner_block);
        let [b_time_text_area, b_time_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(3)])
                .areas(b_time_inner_block);

        Line::from(state.b_water_g_buffer.clone()).render(b_water_text_area, buf);
        Line::from(" g ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(b_water_unit_area, buf);
        Line::from(state.b_time_s_buffer.clone()).render(b_time_text_area, buf);
        Line::from(" s ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(b_time_unit_area, buf);

        let p_water_inner_block = p_water_block.inner(p_water_area);
        let p_time_inner_block = p_time_block.inner(p_time_area);

        p_water_block.render(p_water_area, buf);
        p_time_block.render(p_time_area, buf);

        let [p_water_text_area, p_water_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(3)])
                .areas(p_water_inner_block);
        let [p_time_text_area, p_time_unit_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(3)])
                .areas(p_time_inner_block);

        Line::from(state.p_water_g_buffer.clone()).render(p_water_text_area, buf);
        Line::from(" g ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(p_water_unit_area, buf);
        Line::from(state.p_time_s_buffer.clone()).render(p_time_text_area, buf);
        Line::from(" s ")
            .style(
                Style::default()
                    .bold()
                    .bg(ColorScheme::CHICORY_COFFEE)
                    .fg(ColorScheme::DARK_IRIS),
            )
            .render(p_time_unit_area, buf);

        let blooming_list_block = Block::default()
            .bg(ColorScheme::CHICORY_COFFEE)
            .fg(ColorScheme::DARK_IRIS)
            .title(Line::from(" Bloomings ").alignment(Alignment::Right).bold())
            .title_bottom(
                Line::from_iter([
                    Span::from(" "),
                    Span::from("^c")
                        .bold()
                        .style(Style::default().bg(ColorScheme::DARK_IRIS))
                        .fg(ColorScheme::CHICORY_COFFEE),
                    Span::from(" to Add "),
                ])
                .alignment(Alignment::Left),
            );

        let blooming_list_inner_block = blooming_list_block.inner(blooming_list_area);
        blooming_list_block.render(blooming_list_area, buf);
        Line::from_iter(
            self.bloomings
                .iter()
                .map(|blooming| {
                    Span::from(format!(
                        " Water: {}g - Time: {}s\n",
                        blooming.water_g, blooming.time_s
                    ))
                })
                .collect::<Vec<Span>>(),
        )
        .render(blooming_list_inner_block, buf);

        let pouring_list_block = Block::default()
            .bg(ColorScheme::CHICORY_COFFEE)
            .fg(ColorScheme::DARK_IRIS)
            .title(Line::from(" Pourings ").alignment(Alignment::Right).bold())
            .title_bottom(
                Line::from_iter([
                    Span::from(" "),
                    Span::from("^c")
                        .bold()
                        .style(Style::default().bg(ColorScheme::DARK_IRIS))
                        .fg(ColorScheme::CHICORY_COFFEE),
                    Span::from(" to Add "),
                ])
                .alignment(Alignment::Left),
            );

        let pouring_list_inner_block = pouring_list_block.inner(pouring_list_area);
        pouring_list_block.render(pouring_list_area, buf);
        Line::from_iter(
            self.pourings
                .iter()
                .map(|pouring| {
                    Span::from(format!(
                        " Water: {}g - Time: {}s\n",
                        pouring.water_g, pouring.time_s
                    ))
                })
                .collect::<Vec<Span>>(),
        )
        .render(pouring_list_inner_block, buf);
    }
}

fn is_valid_int_char(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_valid_float_char(buffer: &str, c: char) -> bool {
    c.is_ascii_digit() || (c == '.' && !buffer.contains('.'))
}
