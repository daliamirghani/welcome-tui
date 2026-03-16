use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, widgets::{Block, Borders, Paragraph, Widget}};
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

    fn traverse(&mut self, key_event: KeyEvent){
        let total= self.all_apps.len();
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Up{ 
            if self.selected_index!=0 {self.selected_index -= 1};
        }
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Down{ 
            if self.selected_index +1 < total {self.selected_index += 1};
        }
    }
    
    fn select(&mut self, key_event: KeyEvent){
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter{ 
            self.all_apps[self.selected_index].selected=true;
            // tfw ownership
            let app = self.all_apps[self.selected_index].clone();
            self.selected_items.push(app);
            self.all_apps[self.selected_index].selected = true;
        }
    }
}