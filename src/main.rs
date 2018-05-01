extern crate midir;
extern crate fluidsynth;

use std::io::stdin;
use std::error::Error;
use midir::{MidiInput};
pub mod keyboard;
pub mod menu;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}


fn run() -> Result<(), Box<Error>> {
    let mut keyboard = keyboard::Keyboard::new();
    let mut menu = menu::Menu::new();

    //menu.printMenu();
    //menu.getChoice(&mut keyboard);
    keyboard.add_soundfont(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    keyboard.add_soundfont(".\\soundfonts\\Instruments\\ANCR I Bass Elec 0.sf2", 0, 60, 60);
    keyboard.add_soundfont(".\\soundfonts\\Percussion\\ANCR P Kick 4.sf2", 36, 37, 36);
    keyboard.add_soundfont(".\\soundfonts\\Percussion\\ANCR P Hat 14.sf2", 37, 38, 37);
    keyboard.add_soundfont(".\\soundfonts\\Percussion\\ANCR P Snare 0.sf2", 38, 39, 38);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);

    let mut input = String::new();
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    //midi_in.ignore(Ignore::None);

    println!("Available input ports:");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }
    // print!("Please select input port: ");
    // stdout().flush()?;
    // stdin().read_line(&mut input)?;
    let in_port: usize = 0;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-forward", move |stamp, message, _| {
        keyboard.process(stamp, message);
    }, ())?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connections");
    Ok(())
}
