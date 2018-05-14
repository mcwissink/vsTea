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
        2 - edit SoundFont
        3 - remove SoundFont
        4 - toggle debug
        5 - exit");
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
            Ok(2) => self.edit_font(&keyboard),
            Ok(4) => keyboard.lock().unwrap().toggle_debug(),
            Ok(5) => return false,
            _ => println!("Invalid choice"),
        }

        return true;
    }

    fn load_font(&mut self, keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter path to file: ");
        stdout().flush();
        let mut filename = String::new();
        stdin().read_line(&mut filename);
        keyboard.lock().unwrap().add_soundfont(&filename.trim().replace("\"", ""), 0, 127, 60);
    }

    fn choose_font(&mut self) -> usize {
        print!("Select SoundFont: ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);
        let soundfont = input.trim().parse::<usize>();
        match soundfont {
            Ok(font) => return font,
            Err(err) => println!("Invalid SoundFont"),
        }
        return 0;
    }

    fn edit_font(&mut self, keyboard: &Arc<Mutex<Keyboard>>) {
        let font = self.choose_font();
        print!("Choose parameter (min, max, root): ");
        stdout().flush();
        let mut parameter = String::new();
        stdin().read_line(&mut parameter);
        stdout().flush();
        parameter = parameter.trim().to_string();
        match parameter.as_ref() {
            "min" | "max" | "root" => self.edit_font_param(font, &parameter, &keyboard),
            _ => println!("Invalid parameter"),
        }

        //keyboard.lock().unwrap().set_soundfont_partition();
    }

    fn edit_font_param(&mut self, font: usize, parameter: &str, keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter {} value (0-127): ", parameter);
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);
        let value = input.trim().parse::<usize>();
        match value {
            Ok(val) => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, val),
            Err(err) => println!("Invalid value"),
        }
    }

    // fn getMidi(&mut self, mut )
    //     println!("Available input ports:");
    //     for i in 0..midi_in.port_count() {
    //         println!("{}: {}", i, midi_in.port_name(i)?);
    //     }
}
