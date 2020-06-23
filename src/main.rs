#[macro_use]
extern crate glium;

use glium::Surface;
use std::process::exit;


mod window;
mod draw;
mod fourier_transformation;
mod audio_management;
mod frequency_bars;
mod rect;

//TODO Fileselector
//TODO Malen
//TODO Audiodatein lesen
//TODO FFT-Bibliotheken benutzen


fn main() {
    // FFT
    let data: Vec<u8> = vec![1, 2, 3, 4];
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
    // Read audio
    audio_management::play_audio(&args[1]);


    // Start Window
    window::open_window();

}
