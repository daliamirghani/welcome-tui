use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, widgets::{Block, Borders, Paragraph, Widget}};
use ratatui::{DefaultTerminal};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

use crate::button::Button;

pub struct MainMenu {
    pub exit: bool,
    pub button: Button,
}

impl MainMenu {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_input(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_input(&mut self, key_event: KeyEvent) -> std::io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            self.button.press(key_event);
            if key_event.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }
        Ok(())
    }
}

impl Widget for &MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .border_type(ratatui::widgets::BorderType::Double)
            .borders(Borders::ALL);
        block.render(area, buf);

        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let button_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(20), Constraint::Min(0)])
            .split(chunks[3]);

        Paragraph::new(r#" ██████  ███████  ██████     ██████  ██ ███████ ████████ ██████   ██████  
██    ██ ██      ██          ██   ██ ██ ██         ██    ██   ██ ██    ██ 
██    ██ ███████ ██          ██   ██ ██ ███████    ██    ██████  ██    ██ 
██    ██      ██ ██          ██   ██ ██      ██    ██    ██   ██ ██    ██ 
 ██████  ███████  ██████     ██████  ██ ███████    ██    ██   ██  ██████  
                                                                          
                                                                          "#)
        .alignment(Alignment::Center)
        .render(chunks[1], buf);

        Paragraph::new(r#"Welcome to osc-distro!
The linux distribution that is catered to your needs as an OSCian.
To make your description more personal,select the applications that you want to get pre-installed,
We'll take care of it for you!
"#)
        .alignment(Alignment::Center)
        .style(ratatui::style::Style::default().bold())
        .render(chunks[2], buf);

        self.button.render(button_chunks[1], buf);
    }
}