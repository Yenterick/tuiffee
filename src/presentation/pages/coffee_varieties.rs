use capitalize::Capitalize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, List, ListItem, StatefulWidget, Widget},
};

use crate::{
    models::coffee::CoffeeSummary,
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::coffee_varieties_state::CoffeeVarietiesState, theme::color_scheme::ColorScheme,
    },
};

pub struct CoffeeVarieties {
    pub current_items: Vec<CoffeeSummary>,
    pub current_page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl CoffeeVarieties {
    pub fn new() -> Self {
        Self {
            current_items: Vec::new(),
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
                ScreenAction::None
            }

            KeyCode::Char('k') => {
                state.list_state.select_previous();
                ScreenAction::None
            }

            KeyCode::Char('h') => ScreenAction::PreviousPage,

            KeyCode::Char('l') => ScreenAction::NextPage,

            KeyCode::Char('q') => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [list_area, description_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(area);

        Block::new().borders(Borders::RIGHT).render(list_area, buf);

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

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);
    }
}
