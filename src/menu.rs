

/// Menu - keeps data related to the menu
pub mod menu {
    use std::sync::{Arc, Mutex};
    use std::io::{stdin, stdout, Write};
    use keyboard::Keyboard;

    pub fn print_menu() {
        println!("
        1 - load SoundFont
        2 - toggle debug
        3 - exit");
    }

    pub fn get_choice(keyboard: &Arc<Mutex<Keyboard>>) -> bool {
        print_menu();
        print!("vsTea: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let choice = input.trim().parse::<u32>();

        match choice {
            Ok(1) => load_font(&keyboard),
            //Ok(2) => edit_font(&keyboard),
            Ok(2) => keyboard.lock().unwrap().toggle_debug(),
            Ok(3) => return false,
            _ => println!("Invalid choice"),
        }

        return true;
    }

    /// Lets user load specified SoundFont
    pub fn load_font(keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter path to file: ");
        stdout().flush().unwrap();
        let mut filename = String::new();
        stdin().read_line(&mut filename).unwrap();
        let font = keyboard.lock().unwrap().add_soundfont(&filename.trim().replace("\"", ""), 0, 127, 60) - 1;
        set_font_param(font as usize, "min", &keyboard);
        set_font_param(font as usize, "max", &keyboard);
        set_font_param(font as usize, "root", &keyboard);
    }

    // TODO: use this when Keyboard.list_channels is working
    // fn choose_font() -> usize {
    //     print!("Select SoundFont: ");
    //     stdout().flush();
    //     let mut input = String::new();
    //     stdin().read_line(&mut input);
    //     let soundfont = input.trim().parse::<usize>();
    //     match soundfont {
    //         Ok(font) => return font,
    //         Err(err) => println!("Invalid SoundFont"),
    //     }
    //     return 0;
    // }

    // TODO: decide on how to edit partitions, and how they overwrite each other
    // fn edit_font(&mut self, keyboard: &Arc<Mutex<Keyboard>>) {
    //     let font = choose_font();
    //     print!("Choose parameter (min, max, root): ");
    //     stdout().flush();
    //     let mut parameter = String::new();
    //     stdin().read_line(&mut parameter);
    //     stdout().flush();
    //     parameter = parameter.trim().to_string();
    //     match parameter.as_ref() {
    //         "min" | "max" | "root" => edit_font_param(font, &parameter, &keyboard),
    //         _ => println!("Invalid parameter"),
    //     }
    //
    //     //keyboard.lock().unwrap().set_soundfont_partition();
    // }

    /// Sets a partition parameter of a SoundFont
    fn set_font_param(font: usize, parameter: &str, keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter {} value (0-127): ", parameter);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let value = input.trim().parse::<usize>();
        match value {
            Ok(val) => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, val),
            Err(err) => println!("Invalid value - {}", err),
        }
    }

    // TODO: Be able to choose a midi connection
    // fn getMidi(&mut self, mut )
    //     println!("Available input ports:");
    //     for i in 0..midi_in.port_count() {
    //         println!("{}: {}", i, midi_in.port_name(i)?);
    //     }
}
