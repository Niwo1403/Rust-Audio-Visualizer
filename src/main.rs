#[macro_use]
extern crate glium;

use std::process::exit;


mod window;
mod fourier_transformation;
mod audio_management;
mod visualizer;
mod frequency_bars;
mod oscilloscope;
mod waveform;
mod drawable;
mod rect;
mod line;


use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

fn main() {

    //channel erstellen um Daten vom Player zum Visualizer zu bekommen
    let (value_sender, value_receiver): (Sender<f32>, Receiver<f32>) = mpsc::channel();

    // Argumente
    use std::env;
    let args: Vec<String> = env::args().collect();
    // aud -h testen und ggf. Hilfemenu ausgeben
    for arg in &args {
        if *arg == String::from("-h") {
            println!("Aufruf: ausiovisualizer.exe <DATEINAME> <OPTION>\n\tDATEINAME - der Name der Audiodatei\n\tOPTION - zusätzliche Einstellungsmöglichkeit\n");
            exit(0);
        }
    }
    // Überprüfen ob Dateiname vorhanden ist
    if args.len() < 2 {
        println!("Bitte den Dateinamen der Audiodatei als Argument übergeben.");
        exit(1);
    }

    let file_name = args[1].clone();
    thread::spawn( || {
        // Read audio
        let file_name = file_name;  // keep ref
        audio_management::play_audio(&file_name, value_sender);
        // TODO: Close window here, or close window if value_receiver doesn't receive data anymore
    });

    //art der Visualisierung aus Argumenten lesen
    let mut visualizer_type = 1;
    if args.len() > 2 {
        visualizer_type = args[2].clone().parse::<i32>().unwrap();
        if visualizer_type != 1 && visualizer_type != 2 && visualizer_type != 3 {visualizer_type = 1;}
    }

    // Start Window
    window::start_window(value_receiver, visualizer_type);
}