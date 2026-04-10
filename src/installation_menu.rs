use ratatui::{layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, symbols, widgets::{Block, Borders, Paragraph, Widget},style::{Style, Color}};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use std::{process::Command};
use crate::{button::Button};
use serde::Deserialize;
use std::fs;
use ratatui::layout::Direction;

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

    pub fn traverse(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
        let total_selectable = self.all_apps.len() + 1; 
        match key_event.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index + 1 < total_selectable {
                    self.selected_index += 1;
                }
            }
            _ => {}
            }
        }
        if self.selected_index == self.all_apps.len() { // on the enter button
            self.install_button.is_pressed = true;
        } else {
            self.install_button.is_pressed = false;
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
        Self: Sized,
    {
        let (title_area, apps_area, selected_area, button_area) = self.layout(area);

        Paragraph::new("\n \n \n \n \n \n \nInstallation Menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .render(title_area, buf);

        let mut apps_lines = vec![
            String::new(), String::new(),
            "Applications".to_string(),
            "------------".to_string(),
        ];
        for (i, app) in self.all_apps.iter().enumerate() {
            let prefix = if i == self.selected_index { "▶ " } else { "  " };
            apps_lines.push(format!("{}{}", prefix, app.name));
        }
        self.render_panel(apps_area, buf, apps_lines);

        let mut selected_lines = vec![
            String::new(), String::new(),
            "Selected Items".to_string(),
            "------------".to_string(),
        ];
        selected_lines.extend(self.selected_items.iter().map(|app| app.name.clone()));
        self.render_panel(selected_area, buf, selected_lines);

        let button_color = if self.selected_index == self.all_apps.len() {
            Color::Yellow
        } else {
            Color::Gray
        };
        Paragraph::new("\n [ Press Enter to Install ]")
            .alignment(Alignment::Center)
            .style(Style::default().fg(button_color))
            .render(button_area, buf);
    }
}

impl InstallationMenu {
    fn layout(&self, area: Rect) -> (Rect, Rect, Rect, Rect) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(10),
                Constraint::Length(25),
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(30),
                Constraint::Length(2),
                Constraint::Length(30),
                Constraint::Min(0),
            ])
            .split(vertical_chunks[2]);

        (vertical_chunks[1], horizontal_chunks[1], horizontal_chunks[3], vertical_chunks[3])
    }

    fn render_panel(&self, area: Rect, buf: &mut Buffer, lines: Vec<String>) {
        Block::default()
            .border_set(symbols::border::LIGHT_DOUBLE_DASHED)
            .borders(Borders::ALL)
            .render(area, buf);

        Paragraph::new(lines.join("\n"))
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}