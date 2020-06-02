//The main concept of this library is the Source trait, which represents a sound (streaming or not). In order to play a sound, there are three steps:
//to represent a sound in memory we need three characteristics: the frequency, the number of channels,
// and the list of samples

//A Rust object that represents a sound should implement the Source trait.
// The three characteristics that describe a sound are provided through this trait:
//
// The number of channels can be retrieved with channels.
// The frequency can be retrieved with sample_rate.
// The list of values can be retrieved by iterating on the source. The Source trait requires that the Iterator trait be implemented as well.

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


//To control the playback, rodio provides a type named Sink which represents an audio track.
use rodio::Sink;

let device = rodio::default_output_device().unwrap();
let sink = Sink::new(&device);

// Add a dummy source of the sake of the example.
let source = rodio::source::SineWave::new(440);
sink.append(source);