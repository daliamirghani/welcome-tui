use std::io;
use ratatui::{DefaultTerminal, restore};

mod main_menu;
mod button;

use main_menu::MainMenu;
use button::Button;

pub struct App {
    main_menu: MainMenu,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init(); // initiate terminal
    let mut app = App {
        main_menu: MainMenu {
            exit: false,
            button: Button {
                label: "Press ENTER to get started".to_string(),
                is_pressed: false,
            },
        },
    };
    let result = app.main_menu.run(&mut terminal);
    restore();
    result
}