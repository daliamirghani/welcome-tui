use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, symbols, widgets::{Block, Borders, Paragraph, Widget}};
use ratatui::{DefaultTerminal};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

use crate::App;


pub struct InstallationMenu {
   pub all_apps: Vec<AppItem>,    
    pub selected_items: Vec<AppItem>,
    pub selected_index: usize,
    
}
#[derive(Clone)]
pub struct AppItem {
    pub name: String,
    pub category: String,
    pub selected: bool,
    
}

impl InstallationMenu{
    pub fn create_menu() -> InstallationMenu {
    let mut all_apps = Vec::new();
    {
    all_apps.push(AppItem { name: "VS Code".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "Python".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "Node.js".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "React.js".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "Git".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "Vim".to_string(), category: "Development".to_string(), selected: false });
    all_apps.push(AppItem { name: "Nano".to_string(), category: "Development".to_string(), selected: false });

    all_apps.push(AppItem { name: "LibreOffice".to_string(), category: "Productivity".to_string(), selected: false });

    all_apps.push(AppItem { name: "Blender".to_string(), category: "Design".to_string(), selected: false });

    all_apps.push(AppItem { name: "VLC".to_string(), category: "Utilities".to_string(), selected: false });
    all_apps.push(AppItem { name: "Firefox".to_string(), category: "Utilities".to_string(), selected: false });
    all_apps.push(AppItem { name: "Chrome".to_string(), category: "Utilities".to_string(), selected: false });
    }
    InstallationMenu {
        all_apps,
        selected_items: Vec::new(),
        selected_index: 0,
    }
}

    pub fn traverse(&mut self, key_event: KeyEvent){
        let total= self.all_apps.len();
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Up{ 
            if self.selected_index!=0 {self.selected_index -= 1};
        }
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Down{ 
            if self.selected_index +1 < total {self.selected_index += 1};
        }
    }
    
    pub fn select(&mut self, key_event: KeyEvent){
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter{ 
            self.all_apps[self.selected_index].selected=true;
            // tfw ownership
            let app = self.all_apps[self.selected_index].clone();
            self.selected_items.push(app);
            self.all_apps[self.selected_index].selected = true;
        }
    }
}

impl Widget for &InstallationMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {
        let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(50),Constraint::Percentage(50)])
        .split(area);

        let apps_block = Block::default()
        .border_set(symbols::border::LIGHT_DOUBLE_DASHED)
        .borders(Borders::ALL);
        apps_block.render(chunks[0], buf);

        let selected_block = Block::default()
        .border_set(symbols::border::LIGHT_DOUBLE_DASHED)
        .borders(Borders::ALL);
        selected_block.render(chunks[1], buf);
    }
    
}