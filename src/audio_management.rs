//Source: https://docs.rs/rodio/0.11.0/rodio/

use std::fs::File;
use std::io::BufReader;
//use rodio::Source;    //Rust object that represents a sound should implement the Source trait.
use rodio::Sink;       //type Sink controls  playback (represents audio track).
use std::fs::File;     //to read bytes of file

/*to play a sound:
- Create an object that represents the streaming sound. It can be a sine wave, a buffer, a decoder.
- Choose an output with the devices
- Call play_raw(output, source).
 */
pub fn play_audio(arg: &str) {
    println!("Filename: {:?}", arg);
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open(arg).unwrap();  //added
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    //rodio::play_raw(&device, source.convert_samples()); //

    sink.append(source);
}


let mut f = File::open(arg)?;
let mut buffer = Vec::new();
// Read all bytes until EOF in this source, placing them into buf.
// All bytes read from this source will be appended to the specified buffer buf.This function will continuously call read() to append more data to buf until read() returns either Ok(0) or an error of non-ErrorKind::Interrupted kind.
// If successful, this function will return the total number of bytes read.
f.read_to_end(&mut buffer)?;
