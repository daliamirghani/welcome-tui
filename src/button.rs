use ratatui::{widgets::{Paragraph, Block, Borders, Widget}, layout::Alignment, prelude::{Rect, Buffer}, style::{Style, Color}};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

pub struct Button {
    pub label: String,
    pub is_pressed: bool,
}

impl Button {
    pub fn press(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
            self.is_pressed = true;
        }
    }
}

impl Widget for &Button {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let button = Paragraph::new(self.label.as_str())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .style(
                if self.is_pressed {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default().bg(Color::Gray).fg(Color::Black)
                },
            );

        button.render(area, buf);
    }
}