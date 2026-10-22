use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Widget},
};

use crate::presentation::{
    components::confirmation_modal::ConfirmationModal,
    events::confirmation_choice::ConfirmationChoice, state::app_state::AppState,
    theme::color_scheme::ColorScheme,
};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|frame: &mut Frame<'_>| render(frame, &mut app_state))?;

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

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [complete] = Layout::horizontal([Constraint::Fill(1)]).areas(frame.area());

    Block::new()
        .bg(ColorScheme::CHICORY_COFFEE)
        .render(complete, frame.buffer_mut());

    if let Some(modal) = &app_state.confirmation_modal {
        modal.render(complete, frame.buffer_mut());
    }
}
