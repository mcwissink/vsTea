extern crate fluidsynth;

use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

/// SFSynth - a sound font synth that stores necessary information for the synth
struct SFSynth {
    settings: Settings,
    synth: Synth,
    _driver: AudioDriver,
    min: u8,                // The lowest note this soundfont occupies
    max: u8,                // The highest note this soundfont occupies
    root: i32,              // The offset from middle c (note = 60)
}

impl SFSynth {
    pub fn new(min: u8, max: u8, root: i32) -> SFSynth {
        let mut settings = Settings::new();
        let mut synth = Synth::new(&mut settings);
        let _driver = AudioDriver::new(&mut settings, &mut synth);
        SFSynth {
            settings: settings,
            synth: synth,
            _driver: _driver,
            min: min,
            max: max,
            root: 60 - root,
        }
    }

    pub fn load(&self, filename: &str) {
        self.synth.sfload(filename, 1);
    }

    pub fn set_min(&mut self, min: u8) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: u8) {
        self.max = max;
    }

    pub fn set_root(&mut self, root: i32) {
        self.root = 60 - root;
    }

    pub fn note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        self.synth.noteon(channel as i32, (note as i32) + self.root, velocity as i32);
    }

    pub fn note_off(&mut self, channel: u8, note: u8) {
        self.synth.noteoff(channel as i32, (note as i32) + self.root);
    }
}

pub struct Keyboard {
    sustain: bool,              // Whether the pedal is held or not
    partition: Vec<usize>,      // The keyboard partition, 0 means empty
    synths: Vec<SFSynth>,       // Vector of synths
}

unsafe impl Send for Keyboard {}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            sustain: false,
            partition: vec![0; 128],
            synths: Vec::new(),
        }
    }

    pub fn note_on(&mut self, _channel: u8, note: u8, _velocity: u8) {
        println!("{}", self.partition[note as usize]);
        self.synths[self.partition[note as usize] - 1].note_on(_channel, note, _velocity);
    }

    pub fn note_off(&mut self, _channel: u8, note: u8) {
        self.synths[self.partition[note as usize] - 1].note_off(_channel, note);
    }

    pub fn add_synth(&mut self, filename: &str, min: u8, max: u8, root: i32) {
        let sf_synth = SFSynth::new(min, max, root);
        sf_synth.load(filename);
        self.synths.push(sf_synth);

        let index = self.synths.len();
        for i in (min as usize)..(max as usize) {
            self.partition[i] = index;
        }
    }
}
