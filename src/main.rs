#[macro_use]
extern crate glium;

use glium::Surface;
use std::process::exit;


mod window;
mod draw;
mod fourier_transformation;
mod audio_management;

//TODO Fileselector
//TODO Malen
//TODO Audiodatein lesen
//TODO FFT-Bibliotheken benutzen


fn main() {
    // Read audio
    audio_management::main();

    // FFT
    let mut data = fourier_transformation::example_vec();
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
    println!("Filename: {:?}", args[1]);

    // Start Window
    window::openWindow();

}
