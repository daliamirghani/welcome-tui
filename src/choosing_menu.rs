use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect}, widgets::{Block, Borders, Paragraph, Widget}};
use ratatui::{DefaultTerminal};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};


pub struct ChoosingMenu {
    pub development: Vec<AppItem>,
    pub productivity: Vec<AppItem>,
    pub design: Vec<AppItem>,
    pub utilities: Vec<AppItem>,
    
}
pub struct AppItem {
    pub name: String,
    pub selected: bool,
}
// let menu = Menu {
//     development: vec![
//         AppItem { name: "VS Code".to_string(), selected: false },
//         AppItem { name: "Python".to_string(), selected: false },
//         AppItem { name: "Node.js".to_string(), selected: false },
//         AppItem { name: "React.js".to_string(), selected: false },
//         AppItem { name: "Git".to_string(), selected: false },
//         AppItem { name: "Vim".to_string(), selected: false },
//         AppItem { name: "Nano".to_string(), selected: false },
//     ],
//     productivity: vec![
//         AppItem { name: "LibreOffice".to_string(), selected: false },
//     ],
//     design: vec![
//         AppItem { name: "Blender".to_string(), selected: false },
//     ],
//     utilities: vec![
//         AppItem { name: "VLC".to_string(), selected: false },
//         AppItem { name: "Firefox".to_string(), selected: false },
//         AppItem { name: "Chrome".to_string(), selected: false },
//     ],
// };