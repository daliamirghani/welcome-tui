use std::{io};
use ratatui::{restore};
use std::{process::Command};

mod main_menu;
mod button;
mod installation_menu;

use main_menu::MainMenu;
use installation_menu::InstallationMenu;
use button::Button;
use main_menu::Page;

pub struct App {
    main_menu: MainMenu,
}


fn main() -> io::Result<()> {

    if let Err(e)= get_sudo(){
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    let mut terminal = ratatui::init(); // initiate terminal
    let mut app = App {
        main_menu: MainMenu {
            exit: false,
            button: Button {
                label: "Press ENTER to get started".to_string(),
                is_pressed: false,
            },
            installation_menu:InstallationMenu::create_menu(),
            current_page:Page::MainMenu,
        },
    };
    let result = app.main_menu.run(&mut terminal);
    restore();
    result
}
fn get_sudo() -> io::Result<()>{
    println!("Enter your password to get started!");
    let status = Command::new("sudo")
        .arg("-v")
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Sudo authentication failed"));
    }

    Ok(())
}