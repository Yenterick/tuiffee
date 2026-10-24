use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Widget},
};

use crate::presentation::{
    components::{confirmation_modal::ConfirmationModal, outline::Outline},
    events::{confirmation_choice::ConfirmationChoice, pages::Pages, screen_action::ScreenAction},
    pages::{coffee_varieties::CoffeeVarieties, home::Home, page::Page},
    state::app_state::AppState,
    theme::color_scheme::ColorScheme,
};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState::new();
    let mut home_page: Home = Home::new();
    let mut coffee_varieties_page = CoffeeVarieties::new();

    loop {
        terminal.draw(|frame: &mut Frame<'_>| {
            render(
                frame,
                &mut app_state,
                &mut home_page,
                &mut coffee_varieties_page,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match (key.code, key.modifiers) {
                (event::KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    app_state.confirmation_modal = Some(ConfirmationModal::new(String::from(
                        "Are you sure you want to exit?",
                    )));
                    continue;
                }

                _ => {}
            }

            match app_state.current_page {
                Pages::Home => match home_page.handle_key(key, &mut app_state.home_state) {
                    ScreenAction::GoTo(page) => app_state.current_page = page,

                    _ => {}
                },

                Pages::CoffeeVarieties => {
                    match coffee_varieties_page
                        .handle_key(key, &mut app_state.coffee_varieties_state)
                    {
                        ScreenAction::Back => app_state.current_page = Pages::Home,

                        _ => {}
                    }
                }

                _ => {}
            }

            if let Some(modal) = &mut app_state.confirmation_modal {
                modal.handle_key(key);

                match modal.choice().unwrap_or(ConfirmationChoice::No) {
                    ConfirmationChoice::Yes => {
                        break;
                    }
                    ConfirmationChoice::No => {
                        app_state.confirmation_modal = None;
                    }
                }
                continue;
            }
        }
    }

    Ok(())
}

fn render(
    frame: &mut Frame,
    app_state: &mut AppState,
    home_page: &mut Home,
    coffee_varieties_page: &mut CoffeeVarieties,
) {
    let [complete] = Layout::horizontal([Constraint::Fill(1)]).areas(frame.area());

    let [content] = Layout::horizontal([Constraint::Fill(1)])
        .margin(1)
        .areas(complete);

    Block::new()
        .bg(ColorScheme::CHICORY_COFFEE)
        .render(complete, frame.buffer_mut());

    Outline.render(complete, frame.buffer_mut());

    match app_state.current_page {
        Pages::Home => home_page.render(content, frame.buffer_mut(), &mut app_state.home_state),

        Pages::CoffeeVarieties => coffee_varieties_page.render(
            content,
            frame.buffer_mut(),
            &mut app_state.coffee_varieties_state,
        ),

        _ => {}
    }

    if let Some(modal) = &app_state.confirmation_modal {
        modal.render(complete, frame.buffer_mut());
    }
}
