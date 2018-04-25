extern crate midir;
extern crate fluidsynth;

use std::io::{stdin, stdout, Write};
use std::error::Error;

pub mod keyboard;
use keyboard::Keyboard;
use midir::{MidiInput, Ignore};
use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}


fn run() -> Result<(), Box<Error>> {
    let mut keyboard = Keyboard::new();
    keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I Bass Elec 0.sf2", 0, 60, 60);
    keyboard.add_synth(".\\soundfonts\\Percussion\\ANCR P Kick 4.sf2", 36, 37, 36);
    keyboard.add_synth(".\\soundfonts\\Percussion\\ANCR P Hat 13.sf2", 37, 38, 37);
    keyboard.add_synth(".\\soundfonts\\Percussion\\ANCR P Snare 0.sf2", 38, 39, 38);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    // keyboard.add_synth(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    let mut settings = Settings::new();

    // // Initialize two synths
    // let mut syn1 = Synth::new(&mut settings);
    // let _adriver = AudioDriver::new(&mut settings, &mut syn1);
    // syn1.sfload(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 1);
    //
    // let mut syn2 = Synth::new(&mut settings);
    // let _adriver2 = AudioDriver::new(&mut settings, &mut syn2);
    // syn2.sfload(".\\soundfonts\\Instruments\\ANCR I Bass Elec 0.sf2", 1);

    let mut sustain = false;

    let mut input = String::new();
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(Ignore::None);

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
        println!("{}: {:?} (len = {})", stamp, message, message.len());
        match message[0] {
            176 => { sustain = 127 == message[2]}
            144 => { keyboard.note_on(0, message[1], message[2]) }
            128 => { keyboard.note_off(0, message[1]) }
            _ => ()
        }
    }, ())?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connections");
    Ok(())
}
