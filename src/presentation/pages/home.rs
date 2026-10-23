use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::presentation::{
    events::screen_action::ScreenAction, pages::page::Page, state::home_state::HomeState,
    theme::color_scheme::ColorScheme,
};

pub struct Home;

impl Home {
    pub fn new() -> Home {
        Home
    }
}

impl Page for Home {
    type State = HomeState;

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

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let logo = r#"
                                                  
                                                  
          ▓▓▓▓▓▓▓▓▓▓▓        ▓▓▓▓▓▓▓▓▓▓▓          
       ▓▓▓▓█  ▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓       
      ▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓      
     ▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓     
     ▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓ ▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓     
     ▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓▓     
     ▓▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓▓     
     ▓▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓▓     
      ▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓      
       ▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓       
        ▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓        
         █▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▓         
           ▓▓▓▓▓▓█▓▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓▓▓           
              ▓▓▓▓ ▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓              
                 ▓▓▓░ ▓▓▓▓▓▓▓▓▓▓▓                 
                                                  
                                                  
        "#;

        let lines: Vec<Line> = logo
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                Line::from(Span::styled(
                    line,
                    Style::default().fg(ColorScheme::BAKER_BREAD),
                ))
            })
            .collect();

        let options = [
            "Prepare Coffee",
            "Cup Coffee",
            "Coffee Varieties",
            "Preparation Methods",
        ];
        let highlight_symbol = " >";

        let logo_width = lines.iter().map(Line::width).max().unwrap_or(0) as u16;
        let logo_height = lines.len() as u16;
        // highlight symbol + leading space + longest option + trailing space
        let list_width = options.iter().map(|option| option.len()).max().unwrap_or(0) as u16
            + highlight_symbol.len() as u16
            + 2;
        let list_height = options.len() as u16;

        let [logo_area, _, list_area] = Layout::vertical([
            Constraint::Length(logo_height),
            Constraint::Length(3),
            Constraint::Length(list_height),
        ])
        .flex(Flex::Center)
        .areas(area);

        let logo_area = logo_area.centered_horizontally(Constraint::Length(logo_width));
        let list_area = list_area.centered_horizontally(Constraint::Length(list_width));

        let list = List::new(
            options
                .iter()
                .map(|option| ListItem::from(format!(" {} ", option))),
        )
        .style(Style::default().fg(ColorScheme::DARK_IRIS))
        .highlight_symbol(highlight_symbol)
        .highlight_style(
            Style::default()
                .bold()
                .fg(ColorScheme::CHICORY_COFFEE)
                .bg(ColorScheme::DARK_IRIS),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);

        Paragraph::new(lines).render(logo_area, buf)
    }
}
