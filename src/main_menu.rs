use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, widgets::{Block, Borders, Paragraph, Widget}};
use ratatui::{DefaultTerminal};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

use crate::installation_menu::InstallationMenu;
use crate::button::Button;
pub enum Page {
    MainMenu,
    InstallationMenu,
}
pub struct MainMenu {
    pub exit: bool,
    pub button: Button,
    pub current_page:Page,
    pub installation_menu:InstallationMenu
    
}

impl MainMenu {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_input(key_event)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match self.current_page{
            Page::MainMenu=> {
                frame.render_widget(self, frame.area()); }
            Page::InstallationMenu =>{frame.render_widget(&self.installation_menu, frame.area());}
                
    }}

    fn handle_key_input(&mut self, key_event: KeyEvent) -> std::io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code{
                KeyCode::Esc | KeyCode::Char('q') =>{
                    self.exit = true;
                }
                _ =>{}
            }
            match self.current_page{
                Page::MainMenu =>{
                    if key_event.code == KeyCode::Enter {
                    self.current_page = Page::InstallationMenu;
                }
                }
                Page::InstallationMenu=> {
                    self.installation_menu.traverse(key_event);
                   if  self.installation_menu.select(key_event){
                   { println!("Installation complete");
                    self.exit = true;}
                   }

                }
            }
            
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
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        let button_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(30), Constraint::Min(0)])
            .split(chunks[3]);

        self.render_title(chunks[1], buf);
        self.render_body(chunks[2], buf);
        self.render_button(button_chunks[1], buf);
        Paragraph::new("Press 'q' to exit")
        .alignment(Alignment::Center)
        .render(chunks[5], buf);
}}

impl MainMenu {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(r#"                                                                                          
                                         ▄▄▄▄         ██                                  
                                         ▀▀██         ▀▀                                  
  ▄████▄   ▄▄█████▄   ▄█████▄              ██       ████     ██▄████▄  ██    ██  ▀██  ██▀ 
 ██▀  ▀██  ██▄▄▄▄ ▀  ██▀    ▀              ██         ██     ██▀   ██  ██    ██    ████   
 ██    ██   ▀▀▀▀██▄  ██                    ██         ██     ██    ██  ██    ██    ▄██▄   
 ▀██▄▄██▀  █▄▄▄▄▄██  ▀██▄▄▄▄█              ██▄▄▄   ▄▄▄██▄▄▄  ██    ██  ██▄▄▄███   ▄█▀▀█▄  
   ▀▀▀▀     ▀▀▀▀▀▀     ▀▀▀▀▀                ▀▀▀▀   ▀▀▀▀▀▀▀▀  ▀▀    ▀▀   ▀▀▀▀ ▀▀  ▀▀▀  ▀▀▀ 
                                                                                          
                                                                                          "#)
        .alignment(Alignment::Center)
        .render(area, buf);
    }

    fn render_body(&self, area: Rect, buf: &mut Buffer) {
        let text = "Welcome to osc-linux!\n\
                    The linux distribution that is catered to your needs as an OSCian.\n\
                    To make your description more personal, select the applications that you want to get pre-installed,\n\
                    We'll take care of it for you!";
                    
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(ratatui::style::Style::default().bold())
            .render(area, buf);
    }

    fn render_button(&self, area: Rect, buf: &mut Buffer) {
        self.button.render(area, buf);
    }
}