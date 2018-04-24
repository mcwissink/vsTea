extern crate midir;
extern crate fluidsynth;

use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::vec;

use midir::{MidiInput, MidiOutput, Ignore};
use fluidsynth::{settings, synth, audio};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

// TODO: finish keyboard class
// Will store all the information regarding open synths and partitioning
struct Keyboard {
    sustain: bool,
    fonts: Vec<i32> // TODO: replace with actual data structure
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            sustain: false,
            fonts: Vec::new(),
        }
    }
}


fn run() -> Result<(), Box<Error>> {
    let mut settings = settings::Settings::new();

    // Initialize two synths
    let mut syn1 = synth::Synth::new(&mut settings);
    let _adriver = audio::AudioDriver::new(&mut settings, &mut syn1);
    syn1.sfload(".\\piano2.sf2", 1);

    let mut syn2 = synth::Synth::new(&mut settings);
    let _adriver2 = audio::AudioDriver::new(&mut settings, &mut syn2);
    syn2.sfload(".\\piano.sf2", 1);

    let mut sustain = false;

    let mut input = String::new();
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir forwarding output")?;

    println!("Available input ports:");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }
    print!("Please select input port: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let in_port: usize = input.trim().parse()?;

    println!("\nOpening connections");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-forward", move |stamp, message, _| {
        println!("{}: {:?} (len = {})", stamp, message, message.len());
        match message[0] {
            176 => { sustain = (127 == message[2])}
            144 => { if message[1] > 60 {syn1.noteon(0, message[1] as i32,  message[2] as i32); } else { syn2.noteon(0, message[1] as i32, message[2] as i32);} }
            128 => { if !sustain { if message[1] > 60 {syn1.noteoff(0, message[1] as i32); } else { syn2.noteoff(0, message[1] as i32); } } }
            _ => ()
        }
    }, ())?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connections");
    Ok(())
}
