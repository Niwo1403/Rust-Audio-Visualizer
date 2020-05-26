//The main concept of this library is the Source trait, which represents a sound (streaming or not). In order to play a sound, there are three steps:
//
// Create an object that represents the streaming sound. It can be a sine wave, a buffer, a decoder, etc. or even your own type that implements the Source trait.
// Choose an output with the devices or default_output_device functions.
// Call play_raw(output, source).

use std::fs::File;
use std::io::BufReader;
use rodio::Source;

let device = rodio::default_output_device().unwrap();

let file = File::open("sound.ogg").unwrap();
let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
rodio::play_raw(&device, source.convert_samples());