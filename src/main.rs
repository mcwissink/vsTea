extern crate midir;
extern crate fluidsynth;

use std::sync::{Arc, Mutex};
use std::io::stdin;
use std::error::Error;
use midir::{MidiInput};
pub mod keyboard;
//pub mod menu;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}


fn run() -> Result<(), Box<Error>> {
    let keyboard = Arc::new(Mutex::new(keyboard::Keyboard::new()));

    //let mut keyboard = keyboard::Keyboard::new();
    //let mut menu = menu::Menu::new();
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Instruments\\ANCR I Bass Elec 0.sf2", 0, 60, 60);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Kick 4.sf2", 36, 37, 36);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Hat 14.sf2", 37, 38, 37);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Snare 0.sf2", 38, 39, 38);
    //keyboard.load_soundfont();
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);

    let midi_in = MidiInput::new("in")?;
    //midi_in.ignore(Ignore::None);
    // Assume we are using the first port
    println!("using port: {}", midi_in.port_name(0)?);
    // print!("Please select input port: ");
    // stdout().flush()?;
    // stdin().read_line(&mut input)?;

    // Create a reference to the keyboard so callback thread and main thread can access keyboard_ref
    let shared_keyboard = keyboard.clone();
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(0, "midi", move |stamp, message, _| {
        shared_keyboard.lock().unwrap().process(stamp, message);
    }, ())?;

    //keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    let mut _input = String::new();
    stdin().read_line(&mut _input)?; // wait for next enter key press
    println!("Closing connections");
    Ok(())
}
