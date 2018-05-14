/// Menu - functionality for the menu
pub mod menu {
    use std::sync::{Arc, Mutex};
    use std::io::{stdin, stdout, Write};
    use keyboard::Keyboard;

    pub fn print_menu() {
        println!("
        1 - load SoundFont
        2 - edit SoundFont
        3 - toggle debug
        4 - exit");
    }

    pub fn get_choice(keyboard: &Arc<Mutex<Keyboard>>) -> bool {
        print_menu();
        print!("vsTea: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.to_lowercase().trim().to_string();
        let num_choice = input.parse::<u32>();

        match num_choice {
            Ok(1) => load_font(&keyboard),
            Ok(2) => edit_font(&keyboard),
            Ok(3) => keyboard.lock().unwrap().toggle_debug(),
            Ok(4) => return false,
            _ => {
                match input.as_ref() {
                    "load" => load_font(&keyboard),
                    "edit" => edit_font(&keyboard),
                    "debug" => keyboard.lock().unwrap().toggle_debug(),
                    "quit" | "exit" => return false,
                    _ => println!("Invalid Choice"),
                }
            },
        }

        return true;
    }

    /// Lets user load specified SoundFont
    pub fn load_font(keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter path to file: ");
        stdout().flush().unwrap();
        let mut filename = String::new();
        stdin().read_line(&mut filename).unwrap();
        let font = keyboard.lock().unwrap().add_soundfont(&filename.trim().replace("\"", "")) - 1;
        set_font_param(font as usize, "min", &keyboard);
        set_font_param(font as usize, "max", &keyboard);
        set_font_param(font as usize, "root", &keyboard);
    }

    fn choose_font() -> usize {
        print!("Select SoundFont: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let soundfont = input.trim().parse::<usize>();
        match soundfont {
            Ok(font) => return font,
            Err(err) => println!("Invalid SoundFont - {}", err),
        }
        return 0;
    }

    fn edit_font(keyboard: &Arc<Mutex<Keyboard>>) {
        let font = choose_font();
        print!("Choose parameter (min, max, root): ");
        stdout().flush().unwrap();
        let mut parameter = String::new();
        stdin().read_line(&mut parameter).unwrap();
        parameter = parameter.trim().to_string();
        match parameter.as_ref() {
            "min" | "max" | "root" => set_font_param(font, &parameter, &keyboard),
            _ => println!("Invalid parameter"),
        }
    }

    /// Sets a partition parameter of a SoundFont
    fn set_font_param(font: usize, parameter: &str, keyboard: &Arc<Mutex<Keyboard>>) {
        print!("Enter {} value (0-127): ", parameter);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "" {
            match parameter {
                "min" => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, 0),
                "max" => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, 127),
                "root" => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, 60),
                _ => println!("Invalid parameter"),
            }
        } else {
            let value = input.parse::<usize>();
            match value {
                Ok(val) => keyboard.lock().unwrap().set_soundfont_partition(font, parameter, val),
                Err(err) => println!("Invalid value - {}", err),
            }
        }

    }

    // TODO: Be able to choose a midi connection
    // fn getMidi()
    //     println!("Available input ports:");
    //     for i in 0..midi_in.port_count() {
    //         println!("{}: {}", i, midi_in.port_name(i)?);
    //     }
}
