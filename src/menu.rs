use std::io::{stdin};
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

    pub fn printMenu(&mut self) {
        println!("Welcome to vsTea
        1 - choose MIDI input
        2 - load SoundFont
        3 - edit SoundFont");
    }

    pub fn getChoice(&mut self, mut keyboard: &mut Keyboard) {
        let mut input = String::new();
        stdin().read_line(&mut input);

        let choice = input.trim().parse::<u32>();

        match choice {
            Ok(1) => self.loadFont(&mut keyboard),
            Ok(2) => self.loadFont(&mut keyboard),
            _ => println!("Invalid choice"),
        }
    }

    fn loadFont(&mut self, mut keyboard: &mut Keyboard) {
        let mut filename = String::new();
        stdin().read_line(&mut filename);
        keyboard.add_soundfont(&filename.trim(), 0, 127, 60);
    }

    // fn getMidi(&mut self, mut )
    //     println!("Available input ports:");
    //     for i in 0..midi_in.port_count() {
    //         println!("{}: {}", i, midi_in.port_name(i)?);
    //     }
}
