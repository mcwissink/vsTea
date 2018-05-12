use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use keyboard::Keyboard;

/// Menu - keeps data related to the menu
pub struct Menu {
    directory: &'static str,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            directory: "",
        }
    }

    pub fn print_menu(&mut self) {
        println!("
        1 - load SoundFont
        2 - remove SoundFont
        3 - exit");
    }

    pub fn get_choice(&mut self, keyboard: &Arc<Mutex<Keyboard>>) -> bool {
        self.print_menu();
        print!("vsTea: ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);

        let choice = input.trim().parse::<u32>();

        match choice {
            Ok(1) => self.load_font(&keyboard),
            Ok(2) => self.load_font(&keyboard),
            Ok(3) => return false,
            _ => println!("Invalid choice"),
        }

        return true;
    }

    fn load_font(&mut self, keyboard: &&Arc<Mutex<Keyboard>>) {
        print!("Enter path to file: ");
        stdout().flush();
        let mut filename = String::new();
        stdin().read_line(&mut filename);
        keyboard.lock().unwrap().add_soundfont(&filename.trim().replace("\"", ""), 0, 127, 60);
    }

    // fn getMidi(&mut self, mut )
    //     println!("Available input ports:");
    //     for i in 0..midi_in.port_count() {
    //         println!("{}: {}", i, midi_in.port_name(i)?);
    //     }
}
