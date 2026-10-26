use capitalize::Capitalize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::{
    models::coffee::{Coffee, CoffeeSummary},
    presentation::{
        components::add_coffee_variety_modal::AddCoffeeVarietyModal,
        events::screen_action::ScreenAction, pages::page::Page,
        state::coffee_varieties_state::CoffeeVarietiesState, theme::color_scheme::ColorScheme,
    },
};

pub struct CoffeeVarieties {
    pub current_item: Option<Coffee>,
    pub current_items: Vec<CoffeeSummary>,
    pub add_coffee_variety_modal: Option<AddCoffeeVarietyModal>,
    pub current_page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl CoffeeVarieties {
    pub fn new() -> Self {
        Self {
            current_item: None,
            current_items: Vec::new(),
            add_coffee_variety_modal: None,
            current_page: 1,
            page_size: 10,
            total_pages: 1,
        }
    }
}

impl Page for CoffeeVarieties {
    type State = CoffeeVarietiesState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match key.code {
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

            KeyCode::Char('q') => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [list_area, description_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(area);

        Block::new().borders(Borders::RIGHT).render(list_area, buf);

        let [inner_list, bottom_title] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(list_area);

        let list = List::new(self.current_items.iter().map(|item: &CoffeeSummary| {
            ListItem::from(format!(
                " {} by {} - {} {}",
                item.name.as_deref().unwrap_or("Coffee"),
                item.brand,
                item.process_type.capitalize(),
                item.variety_name
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

        if let Some(coffee) = &self.current_item {
            let coffee_description = format!(
                " Descriptive Notes: {}\n Location: {}\n Roast Level: {}\n Process Type: {}",
                coffee
                    .descriptive_notes
                    .as_deref()
                    .unwrap_or("Unknown")
                    .capitalize(),
                coffee.location.as_deref().unwrap_or("Unknown").capitalize(),
                coffee.roasting_level.capitalize(),
                coffee.process_type.capitalize()
            );

            Paragraph::new(coffee_description.as_str()).render(description_area, buf);
        }

        StatefulWidget::render(list, inner_list, buf, &mut state.list_state);

        Paragraph::new(format!("Current Page: {}", self.current_page))
            .centered()
            .style(Style::default().bold())
            .render(bottom_title, buf);
    }
}
