extern crate midir;
extern crate fluidsynth;
extern crate piston_window;

use piston_window::*;
use std::sync::{Arc, Mutex};
use std::io::stdin;
use std::error::Error;
use std::thread;
use midir::{MidiInput};
mod keyboard;
mod menu;


fn main() {
    // Notes is going to be shared between threads so it is necessary to wrap it in a Arc and Mutex to ensure thread safety
    let mut notes: Vec<f32> = vec![0.005; 128];
    let notes1 = Arc::new(Mutex::new(notes));
    let notes2 = notes1.clone();
    // Spawn the command line thread
    let cl = thread::spawn(move || {
        match run(&notes1) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err.description())
        }
    });

    // Start our gui window
    let opengl = OpenGL::V3_2;
    let (width, height) = (16*50, 8*50);
    let mut window: PistonWindow =
        WindowSettings::new("vsTea", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                // Unlock notes2 once so we don't have to lock it 128 times
                let notes3 = notes2.lock().unwrap();
                for i in 0..16 {
                    for j in 0..8 {
                        let transform = c.transform.trans(50.0 * i as f64, 50.0 * j as f64);
                        rectangle([1.0, 1.0, 1.0, notes3[i + j * 16] as f32],
                                  [5.0, 5.0, 45.0, 45.0],
                                  transform, g);
                    }
                }
            });
        }
    }
    // Join our thread
    cl.join().unwrap();
}


fn run(notes: &Arc<Mutex<Vec<f32>>>) -> Result<(), Box<Error>> {
    // Bind the notes to esnure the reference
    let notes1 = notes.clone();
    // Create a new keyboard and menu instance
    // We need to protect the keyboard with an Arc and Mutex
    // Arc keeps track of how many references and calls destructor accordingly
    // Mutex makes sure both threads aren't access the keyboard at the same time
    let keyboard = Arc::new(Mutex::new(keyboard::Keyboard::new()));
    let mut menu = menu::Menu::new();

    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Instruments\\ANCR I E Piano 15.sf2", 0, 127, 60);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Instruments\\ANCR I Bass Elec 0.sf2", 0, 60, 60);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Kick 4.sf2", 36, 37, 36);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Hat 14.sf2", 37, 38, 37);
    keyboard.lock().unwrap().add_soundfont(".\\soundfonts\\Percussion\\ANCR P Snare 0.sf2", 38, 39, 38);
    keyboard.lock().unwrap().partition_all();

    let midi_in = MidiInput::new("in")?;
    // Assume we are using the first port
    println!("using port: {}", midi_in.port_name(0)?);

    // Create a reference to the keyboard so callback thread and main thread can access keyboard_ref
    let shared_keyboard = keyboard.clone();
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(0, "midi", move |stamp, message, _| {
        shared_keyboard.lock().unwrap().process(stamp, message, &notes1);
    }, ())?;

    // Start the menu
    println!("Welcome to vsTea");
    while menu.get_choice(&keyboard) { /* Wait for user to exit menu */ }

    // End the process
    println!("Closing connections");
    Ok(())
}
