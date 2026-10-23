use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Widget},
};

use crate::presentation::theme::color_scheme::ColorScheme;

pub struct Outline;

impl Widget for Outline {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Plain)
            .fg(ColorScheme::DARK_IRIS)
            .title(
                Line::from_iter([Span::from(" TUIFFEE ")])
                    .bold()
                    .alignment(Alignment::Right),
            )
            .title_bottom(Line::from_iter([
                Span::from(" ^q "),
                Span::from("exit ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" q "),
                Span::from("back ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" h "),
                Span::from("left ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" j "),
                Span::from("down ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" k "),
                Span::from("up ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" l "),
                Span::from("right ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" ↵ "),
                Span::from("confirm ")
                    .bold()
                    .fg(ColorScheme::CHICORY_COFFEE)
                    .bg(ColorScheme::DARK_IRIS),
                Span::from(" "),
            ]))
            .render(area, buf);
    }
}
