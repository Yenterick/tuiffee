use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    models::coffee::Coffee,
    presentation::{
        events::screen_action::ScreenAction, pages::page::Page,
        state::coffee_varieties_state::CoffeeVarietiesState,
    },
};

pub struct CoffeeVarieties {
    pub current_items: Vec<Coffee>,
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

            KeyCode::Char('h') => ScreenAction::PrevPage,

            KeyCode::Char('l') => ScreenAction::NextPage,

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
