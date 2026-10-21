use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Widget},
};

use crate::presentation::theme::color_scheme::ColorScheme;

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame: &mut Frame<'_>| render(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match (key.code, key.modifiers) {
                (event::KeyCode::Char('q'), KeyModifiers::CONTROL) => continue,

                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame) {
    let [complete] = Layout::horizontal([Constraint::Fill(1)]).areas(frame.area());

    Block::new()
        .bg(ColorScheme::DARK_ORCHESTRA)
        .render(complete, frame.buffer_mut());
}
