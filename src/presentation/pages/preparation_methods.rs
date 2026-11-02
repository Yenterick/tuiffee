use capitalize::Capitalize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::{
    models::preparation_technique::{FullPreparationTechnique, PreparationTechniqueSummary},
    presentation::{
        components::add_preparation_technique_modal::AddPreparationTechniqueModal,
        events::screen_action::ScreenAction,
        pages::page::Page,
        state::{
            add_preparation_technique_modal_state::AddPreparationTechniqueModalState,
            preparation_methods_state::PreparationMethodState,
        },
        theme::color_scheme::ColorScheme,
    },
};

pub struct PreparationMethods {
    pub current_item: Option<FullPreparationTechnique>,
    pub current_items: Vec<PreparationTechniqueSummary>,
    pub current_page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl PreparationMethods {
    pub fn new() -> Self {
        Self {
            current_item: None,
            current_items: Vec::new(),
            current_page: 1,
            page_size: 10,
            total_pages: 1,
        }
    }
}

impl Page for PreparationMethods {
    type State = PreparationMethodState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        let modal_action = if let Some(modal) = &mut state.add_preparation_technique_modal
            && let Some(modal_state) = &mut state.add_preparation_technique_modal_state
        {
            Some(modal.handle_key(key, modal_state))
        } else {
            None
        };

        if let Some(action) = modal_action {
            return match action {
                ScreenAction::Back => {
                    state.add_preparation_technique_modal = None;
                    state.add_preparation_technique_modal_state = None;

                    ScreenAction::None
                }

                _ => ScreenAction::None,
            };
        }

        match key.code {
            KeyCode::Char('A') => {
                state.add_preparation_technique_modal = Some(AddPreparationTechniqueModal::new());
                state.add_preparation_technique_modal_state =
                    Some(AddPreparationTechniqueModalState::new());
                ScreenAction::None
            }

            KeyCode::Char('q') => ScreenAction::Back,

            KeyCode::Char('j') => {
                state.list_state.select_next();
                if state.list_state.selected().unwrap_or(0) < self.current_items.len() {
                    ScreenAction::LoadId(
                        self.current_items[state.list_state.selected().unwrap_or(0)]
                            .id
                            .unwrap_or(0),
                    )
                } else {
                    ScreenAction::None
                }
            }

            KeyCode::Char('k') => {
                state.list_state.select_previous();
                ScreenAction::LoadId(
                    self.current_items[state.list_state.selected().unwrap_or(0)]
                        .id
                        .unwrap_or(0),
                )
            }

            KeyCode::Char('h') => {
                if self.current_page > 1 {
                    ScreenAction::PreviousPage
                } else {
                    ScreenAction::None
                }
            }

            KeyCode::Char('l') => {
                if self.current_items.len() == self.page_size as usize {
                    ScreenAction::NextPage
                } else {
                    ScreenAction::None
                }
            }

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

        let list = List::new(self.current_items.iter().map(|item| {
            ListItem::from(format!(
                " {} for {} {}",
                item.name,
                item.people_target,
                String::from(if item.people_target > 1 {
                    "people"
                } else {
                    "person"
                })
            ))
        }))
        .style(Style::default().fg(ColorScheme::DARK_IRIS))
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bold()
                .fg(ColorScheme::CHICORY_COFFEE)
                .bg(ColorScheme::DARK_IRIS),
        );

        if let Some(preparation_technique) = &self.current_item {
            let blooming_description = preparation_technique
                .bloomings
                .iter()
                .map(|blooming| {
                    format!(
                        "No: {} - Water Level: {}g - Time: {}s",
                        blooming.sequency, blooming.water_g, blooming.time_s
                    )
                })
                .collect::<Vec<_>>()
                .join("\n ");
            let pouring_description = preparation_technique
                .pourings
                .iter()
                .map(|pouring| {
                    format!(
                        "No: {} - Water Level: {}g - Time: {}s",
                        pouring.sequency, pouring.water_g, pouring.time_s
                    )
                })
                .collect::<Vec<_>>()
                .join("\n ");
            let preparation_description = format!(
                " Grind Level: {}\n Ratio: 1:{}\n Coffee: {}g\n Bloomings: \n {} \n Pourings: \n {}",
                preparation_technique.grind_level.capitalize(),
                preparation_technique.ratio,
                preparation_technique.coffee_g,
                blooming_description,
                pouring_description
            );

            Paragraph::new(preparation_description.as_str()).render(description_area, buf);
        }

        StatefulWidget::render(list, inner_list, buf, &mut state.list_state);

        Paragraph::new(format!("Current Page: {}", self.current_page))
            .centered()
            .style(Style::default().bold())
            .render(bottom_title, buf);

        Line::from_iter([
            Span::from("Press "),
            Span::from("'A'")
                .bold()
                .style(Style::default().bg(ColorScheme::DARK_IRIS))
                .fg(ColorScheme::CHICORY_COFFEE),
            Span::from(" to Add a New Preparation Technique "),
        ])
        .centered()
        .render(add_button, buf);

        if let Some(modal) = &state.add_preparation_technique_modal
            && let Some(state) = &mut state.add_preparation_technique_modal_state
        {
            modal.render(area, buf, state);
        }
    }
}
