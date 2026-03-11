use std::{io, path::Path};
use ratatui::{layout::Margin, style::Style};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Alignment, Constraint, Layout}, restore, style::Stylize, text::Line, widgets::{Block, Borders, Padding, Paragraph, Widget}};

fn main()-> io::Result<()> {
    let mut terminal = ratatui::init(); //initiated terminal
    let mut app = App {main_menu:MainMenu { exit:false, button:Button {label:"Let's go".to_string(), is_pressed:false} }}; //initiated our application
    let result= app.main_menu.run(&mut terminal);
    ratatui::restore(); 
    result
}

pub struct App{
    main_menu:MainMenu
}
pub struct MainMenu{
    exit:bool,
    button:Button
}
pub struct Button{
    label:String,
    is_pressed:bool
}
impl Button{
    fn if_pressed(){
        //new scene logic whatever
    }
}
impl Widget for &Button{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let block= Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(ratatui::style::Color::LightYellow).fg(ratatui::style::Color::Green));
    block.render(area, buf);
    }
}
impl MainMenu{
    fn run(&mut self,terminal:&mut DefaultTerminal)->io::Result<()>{
        while !self.exit{
            match crossterm::event::read()?{
                crossterm::event::Event::Key(key_event)=> self.handle_key_input(key_event)?,
                _=>{}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }
    fn draw(&self, frame:&mut Frame){
        frame.render_widget(self, frame.area());
    }
    fn handle_key_input(&mut self, key_event:crossterm::event::KeyEvent)-> io::Result<()>{
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q'){
            self.exit = true;
        }
        Ok(())
    }

}

impl Widget for &MainMenu {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let block =Block::default()
        .border_type(ratatui::widgets::BorderType::Double)
        .borders(Borders::all());
        block.render(area, buf);
        

        let chunks =Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        // i made these constraints such that the text is centered
        .constraints([Constraint::Min(0),Constraint::Length(10),Constraint::Length(10),Constraint::Length(3), Constraint::Min(0)])
        .split(area);

        let name= Paragraph::new(r#" ██████  ███████  ██████     ██████  ██ ███████ ████████ ██████   ██████  
██    ██ ██      ██          ██   ██ ██ ██         ██    ██   ██ ██    ██ 
██    ██ ███████ ██          ██   ██ ██ ███████    ██    ██████  ██    ██ 
██    ██      ██ ██          ██   ██ ██      ██    ██    ██   ██ ██    ██ 
 ██████  ███████  ██████     ██████  ██ ███████    ██    ██   ██  ██████  
                                                                          
                                                                          "#)
        .alignment(Alignment::Center)
        .render(chunks[1], buf);
    
        let description = Paragraph::new(r#"Welcome to osc-distro!
        The linux distribution that is catered to your needs as an OSCian.
        To make your description more personal,select the applications that you want to get pre-installed,
        We'll take care of it for you!
        "#).alignment(Alignment::Center)
        .bold()
        .render(chunks[2], buf);
        
        self.button.render(chunks[3], buf);
            
        
    }
}
