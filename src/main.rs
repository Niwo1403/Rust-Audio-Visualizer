#[macro_use]
extern crate glium;

use std::process::exit;


mod window;
mod fourier_transformation;
mod audio_management;
mod frequency_bars;
mod rect;

//TODO Fileselector
//TODO Malen
//TODO Audiodatein lesen
//TODO FFT-Bibliotheken benutzen

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

fn main() {
    let (value_sender, value_receiver): (Sender<f32>, Receiver<f32>) = mpsc::channel();

    // FFT
    let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
    let mut data = fourier_transformation::data_to_c64(data);
    println!("{:?}\n\n", data);
    data = fourier_transformation::transform(data);
    println!("{:?}", data);

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

    // Start Window
    window::start_window(value_receiver);
}