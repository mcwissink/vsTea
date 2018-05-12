extern crate fluidsynth;

use std::io::{stdin};
use std::error::Error;
use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

/// SFParition - a structure that stores necessary information for a partition of a soudnfont
struct SFParition {
    id: u32,                // The id of the soundfont, analgous to the channel that the soundfont is on
    channel: i32,           // channel  = 1 - id
    min: usize,             // The lowest note this soundfont occupies
    max: usize,             // The highest note this soundfont occupies
    root: i32,              // The offset from middle c (note = 60)
}

impl SFParition {
    pub fn new(id: u32, min: usize, max: usize, root: i32) -> SFParition {
        SFParition {
            id: id,
            channel: (id - 1) as i32,
            min: min,
            max: max,
            root: 60 - root,
        }
    }

    pub fn set_min(&mut self, min: usize) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    // pub fn set_root(&mut self, root: i32) {
    //     self.root = 60 - root;
    // }
}

/// Keyboard - stores all the logic for partitioning the keyboard
pub struct Keyboard {
    synth: Synth,
    _settings: Settings,
    _driver: AudioDriver,
    partition: Vec<i32>,             // The keyboard partition, 0 means empty
    soundfonts: Vec<SFParition>,       // Vector of synths
}

unsafe impl Send for Keyboard {} // TODO: guarentee thred safety of Keyboard

impl Keyboard {
    pub fn new() -> Keyboard {
        let mut _settings = Settings::new();
        let mut synth = Synth::new(&mut _settings);
        synth.set_gain(1.0); // TODO: Make this a function
        let _driver = AudioDriver::new(&mut _settings, &mut synth);

        // Initialize our partition vector
        let mut partition: Vec<i32> = Vec::with_capacity(128);
        for _ in 0..127 {
            partition.push(0);
        }

        // Initialize our soundfont vector
        let soundfonts: Vec<SFParition> = Vec::new();

        // Create the keyboard
        Keyboard {
            synth: synth,
            _settings: _settings,
            _driver: _driver,
            partition: partition,
            soundfonts: soundfonts,
        }
    }

    /// Process a midi message
    /// Recieve: _stamp - a time stamp
    ///          message - the midi message
    pub fn process(&mut self, _stamp: u64, message: &[u8]) {
        //println!("{}: {:?} (len = {})", stamp, message, message.len());
        match message[0] {
            176 => { /* Do something with sustain */ }
            144 => { self.note_on(message[1] as i32, message[2] as i32) }
            128 => { self.note_off(message[1] as i32) }
            _ => ()
        }
    }

    pub fn note_on(&mut self, note: i32, velocity: i32) {
        let channel = self.partition[note as usize];
        self.synth.noteon(channel, note + self.soundfonts[channel as usize].root, velocity);
    }

    pub fn note_off(&mut self, note: i32) {
        let channel = self.partition[note as usize];
        self.synth.noteoff(channel, note + self.soundfonts[channel as usize].root);
    }

    pub fn add_soundfont(&mut self, filename: &str, min: usize, max: usize, root: i32) {
        // Load the SoundFont
        let id = self.synth.sfload(filename, 1).unwrap();
        let sf_parition = SFParition::new(id, min, max, root);
        self.soundfonts.push(sf_parition);

        // Remap the SoundFonts to their resepctive channels
        for soundfont in &self.soundfonts {
            self.synth.program_select(soundfont.channel, soundfont.id, 0, 0);
        }

        // Apply the partition
        for i in min..max {
            self.partition[i] = self.soundfonts[(id - 1) as usize].channel;
        }
    }

    pub fn load_soundfont(&mut self) -> Result<(), Box<Error>> {
        println!("Enter path to file: ");
        let mut filename = String::new();
        stdin().read_line(&mut filename)?;
        self.add_soundfont(&filename.trim(), 0, 127, 60);
        Ok(())
    }

    pub fn list_channels(&mut self) {
        //let channels = self.synth.count_midi_channels();
        let info = self.synth.get_channel_info(0);
        match info {
            Some(x) => println!("{}: {}", 0, x.name),
            None    => println!("None"),
        }
    }

    pub fn set_partition_max(&mut self, font: usize) {
        self.synth.get_channel_info(font as i32);
        println!("Enter path to file: ");
        self.soundfonts[font].set_max(0)
    }
}
