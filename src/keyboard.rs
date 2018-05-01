extern crate fluidsynth;

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

    pub fn set_root(&mut self, root: i32) {
        self.root = 60 - root;
    }
}

/// Keyboard - stores all the logic for partitioning the keyboard
pub struct Keyboard {
    settings: Settings,
    synth: Synth,
    _driver: AudioDriver,
    partition: Vec<i32>,             // The keyboard partition, 0 means empty
    soundfonts: Vec<SFParition>,       // Vector of synths
}

unsafe impl Send for Keyboard {} // TODO: guarentee thred safety of Keyboard

impl Keyboard {
    pub fn new() -> Keyboard {
        let mut settings = Settings::new();
        let mut synth = Synth::new(&mut settings);
        synth.set_gain(2.0); // TODO: Make this a function
        let _driver = AudioDriver::new(&mut settings, &mut synth);

        // Initialize our partition vector
        let mut partition: Vec<i32> = Vec::with_capacity(128);
        for i in 0..127 {
            partition.push(0);
        }

        // Initialize our soundfont vector
        let soundfonts: Vec<SFParition> = Vec::new();

        // Create the keyboard
        Keyboard {
            settings: settings,
            synth: synth,
            _driver: _driver,
            partition: partition,
            soundfonts: soundfonts,
        }
    }

    pub fn process(&mut self, stamp: u64, message: &[u8]) {
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
        //self.synths[self.partition[note as usize] - 1].note_on(_channel, note, _velocity);
    }

    pub fn note_off(&mut self, note: i32) {
        let channel = self.partition[note as usize];
        self.synth.noteoff(channel, note + self.soundfonts[channel as usize].root);
        //self.synths[self.partition[note as usize] - 1].note_off(_channel, note);
    }

    pub fn add_soundfont(&mut self, filename: &str, min: usize, max: usize, root: i32) {
        let id = self.synth.sfload(filename, 0).unwrap();
        let sf_parition = SFParition::new(id, min, max, root);
        self.soundfonts.push(sf_parition);

        for soundfont in &self.soundfonts {
            self.synth.program_select(soundfont.channel, soundfont.id, 0, 0);
        }

        // Apply the partition
        for i in min..max {
            self.partition[i] = self.soundfonts[(id - 1) as usize].channel;
        }
    }
}
