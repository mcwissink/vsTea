extern crate fluidsynth;

use std::sync::{Arc, Mutex};
use std::io::{stdin};
use std::error::Error;
use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

/// SFParition - a structure that stores necessary information for a partition of a soudnfont
struct SFParition {
    id: u32,                // The id of the soundfont, analgous to the channel that the soundfont is on
    channel: i32,           // channel = 1 - id
    min: usize,             // The lowest note this soundfont occupies
    max: usize,             // The highest note this soundfont occupies
    root: i32,              // The offset from middle c (note = 60), allows for transposing
}

impl SFParition {
    pub fn new(id: u32, min: usize, max: usize, root: usize) -> SFParition {
        SFParition {
            id: id,
            channel: (id - 1) as i32,
            min: min,
            max: max,
            root: 60 - (root as i32),
        }
    }

    pub fn set_min(&mut self, min: usize) {
        println!("Setting min: {}", min);
        self.min = min;
    }

    pub fn set_max(&mut self, max: usize) {
        println!("Setting max: {}", max);
        self.max = max + 1; // Adding the 1 for user understanding
    }

    pub fn set_root(&mut self, root: usize) {
        println!("Setting root: {}", root);
        self.root = 60 - (root as i32);
    }
}

/// Keyboard - stores all the logic for partitioning the keyboard
pub struct Keyboard {
    synth: Synth,
    _settings: Settings,
    _driver: AudioDriver,
    partition: Vec<i32>,                // The keyboard partition, 0 means empty
    soundfonts: Vec<SFParition>,        // Vector of synths
    debug: bool,                       // Prints midi messages
}

// We need to let our keyboard be shared among threads
// This is unsafe, but our keyboard will be protected by a lock
unsafe impl Send for Keyboard {}

impl Keyboard {
    pub fn new() -> Keyboard {
        let mut _settings = Settings::new();
        let mut synth = Synth::new(&mut _settings);
        synth.set_gain(1.0); // TODO: Make this a function
        let _driver = AudioDriver::new(&mut _settings, &mut synth);

        // Initialize our partition vector
        let mut partition: Vec<i32> = vec![0; 128];

        // Initialize our soundfont vector
        let soundfonts: Vec<SFParition> = Vec::new();

        // Create the keyboard
        Keyboard {
            synth: synth,
            _settings: _settings,
            _driver: _driver,
            partition: partition,
            soundfonts: soundfonts,
            debug: false,
        }
    }

    /// Process a midi message
    /// Receive: stamp - a time stamp
    ///          message - the midi message
    pub fn process(&mut self, stamp: u64, message: &[u8], notes: &Arc<Mutex<Vec<f32>>>) {
        // We ignore message 254 since it is just Active Sensing, it's just the keyboard pinging the computer
        // The message clutters the screen so don't print it
        if self.debug && message[0] != 254 {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        }

        match message[0] {
            176 => { /*TODO: Do something with sustain */ }
            144 => { self.note_on(message[1] as i32, message[2] as i32, &notes) }
            128 => { self.note_off(message[1] as i32, &notes) }
            _ => ()
        }
    }

    /// Set the note value and play the sound
    pub fn note_on(&mut self, note: i32, velocity: i32, notes: &Arc<Mutex<Vec<f32>>>) {
        notes.lock().unwrap()[note as usize] = velocity as f32 / 127.0; // This basically controlls the color of the GUI
        let channel = self.partition[note as usize];
        self.synth.noteon(channel, note + self.soundfonts[channel as usize].root, velocity);
    }

    /// Reset the note value and stop the sound
    pub fn note_off(&mut self, note: i32, notes: &Arc<Mutex<Vec<f32>>>) {
        notes.lock().unwrap()[note as usize] = 0.005;
        let channel = self.partition[note as usize];
        self.synth.noteoff(channel, note + self.soundfonts[channel as usize].root);
    }

    /// Add a SoundFont to the synth
    /// Receive: filename - a path to the SoundFont
    ///          min, max, root: parition values
    /// Return: the SoundFont id
    pub fn add_soundfont(&mut self, filename: &str, min: usize, max: usize, root: usize) -> u32 {
        // Load the SoundFont
        let id = match self.synth.sfload(filename, 1) {
            Some(i) => i,
            None    => 0,
        };

        // Add our partition
        let sf_parition = SFParition::new(id, min, max, root);
        self.soundfonts.push(sf_parition);

        // Remap the SoundFonts to their respective channels
        for font in &self.soundfonts {
            self.synth.program_select(font.channel, font.id, 0, 0);
        }

        return id;
    }

    /// Updates the keyboard partition
    pub fn partition_all(&mut self) {
        for font in &self.soundfonts {
            for i in font.min..font.max {
                self.partition[i] = font.channel;
            }
        }
    }

    /// Only update 1 soundfont partition
    pub fn partition_soundfont(&mut self, font: usize) {
        let soundfont = &self.soundfonts[font];
        for i in soundfont.min..soundfont.max {
            self.partition[i] = soundfont.channel;
        }
    }

    pub fn list_channels(&mut self) {
        //let channels = self.synth.count_midi_channels();
        let info = self.synth.get_channel_info(0);
        match info {
            Some(x) => println!("{}: {}", 0, x.name),
            None    => println!("None"),
        }
    }

    /// Set a partition value for a SoundFont
    pub fn set_soundfont_partition(&mut self, font: usize, parameter: &str, value: usize) {
        {
            let soundfont = &mut self.soundfonts[font];
            match parameter {
                "max" => soundfont.set_max(value),
                "min" => soundfont.set_min(value),
                "root" => soundfont.set_root(value),
                _ => println!("Invalid parameter"),
            }
        }
        self.partition_all();
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }
}
