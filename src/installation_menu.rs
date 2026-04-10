use ratatui::{layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, symbols, widgets::{Block, Borders, Paragraph, Widget},style::{Style, Color}};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use std::{process::Command};
use crate::{button::Button};
use serde::Deserialize;
use std::fs;


pub struct InstallationMenu {
    pub all_apps: Vec<AppItem>,    
    pub selected_items: Vec<AppItem>,
    pub selected_index: usize,
    pub install_button: Button

    
}
#[derive(Clone)]
#[derive(Deserialize)]
pub struct AppItem {
    pub name: String,
    pub actual_name:String, //for installation commands
    pub category: String,
    pub selected: bool,
    
}

impl InstallationMenu{
    pub fn create_menu() -> InstallationMenu {
        let content = fs::read_to_string("apps.yaml").expect("Failed to read file");
        let all_apps:Vec<AppItem> = serde_yaml::from_str(&content).expect("Failed to parse file content");
    InstallationMenu {
        all_apps,
        selected_items: Vec::new(),
        selected_index: 0,
        install_button: Button{label: "Install".to_string(), is_pressed:false},
    }
}

    pub fn traverse(&mut self, key_event: KeyEvent){
        let total= self.all_apps.len()+1;
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Up{ 
            if self.selected_index!=0 {self.selected_index -= 1};
        }
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Down{ 
            if self.selected_index +1 < total {self.selected_index += 1};
        if self.selected_index == self.all_apps.len(){
            self.install_button.is_pressed =true;
        }
        }
    }
    
    pub fn select(&mut self, key_event: KeyEvent)->bool{
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter{ 
            if self.install_button.is_pressed{
                let apps:Vec<String> = self.selected_items.iter().map(|app|app.actual_name.clone()).collect();
                let apps_string = apps.join(" ");
                Command::new("bash")
                .arg("install.sh")
                .arg(apps_string) 
                .status()
                .expect("failed to run install script");
                return true;

            }
            let app: AppItem = self.all_apps[self.selected_index].clone();
            if self.all_apps[self.selected_index].selected != true{
            self.selected_items.push(app);
            self.all_apps[self.selected_index].selected = true;}
        }
    
    false
}
}

impl Widget for &InstallationMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {
        
        let vertical_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(0), 
                Constraint::Length(10), 
                Constraint::Length(25), 
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(area);

        let horizonta_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(0), 
                Constraint::Length(30),
                Constraint::Length(2),  
                Constraint::Length(30), 
                Constraint::Min(0),
            ])
            .split(vertical_chunks[2]);


        let title_block = Paragraph::new("\n \n \n \n \n \n \n \nInstallation Menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        title_block.render(vertical_chunks[1], buf);


        let apps_block = Block::default()
            .border_set(symbols::border::LIGHT_DOUBLE_DASHED)
            .borders(Borders::ALL);
        apps_block.render(horizonta_chunks[1], buf);

        let mut apps_lines = Vec::new();
        let mut selected_lines = Vec::new();

        apps_lines.push("".to_string());  
        apps_lines.push("".to_string());  
        apps_lines.push("Applications".to_string());
        apps_lines.push("------------".to_string());
        apps_lines.push("".to_string());

        for i in 0..self.all_apps.len() {
            let app = &self.all_apps[i];
            let prefix = if i == self.selected_index { "▶ " } else { "  " };
            let line = format!("{}{}", prefix, app.name);
            apps_lines.push(line);
        }
        selected_lines.push("".to_string());  
        selected_lines.push("".to_string());  
        selected_lines.push("Selected Items".to_string());
        selected_lines.push("------------".to_string());
        selected_lines.push("".to_string());

        for app in &self.selected_items {
            let line = format!("{}", app.name);
            selected_lines.push(line);
        }

        Paragraph::new(apps_lines.join("\n"))
            .alignment(Alignment::Center)
            .render(horizonta_chunks[1], buf);

        Paragraph::new(selected_lines.join("\n"))
            .alignment(Alignment::Center)
            .render(horizonta_chunks[3], buf);

        let selected_block = Block::default()
            .border_set(symbols::border::LIGHT_DOUBLE_DASHED)
            .borders(Borders::ALL);
        selected_block.render(horizonta_chunks[3], buf);

        let button_color = if self.selected_index == self.all_apps.len(){
            Color::Yellow
        }
        else {
            Color::Gray
        };

        let button_paragraph = Paragraph::new("\n [ Press Enter to Install ]")
            .alignment(Alignment::Center)
            .style(Style::default().fg(button_color));

        button_paragraph.render(vertical_chunks[3], buf);
    }
}