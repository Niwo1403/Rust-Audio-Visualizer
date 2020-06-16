//Source: https://docs.rs/rodio/0.11.0/rodio/

use std::fs::File;
use std::io::BufReader;
//Rust object that represents a sound should implement the Source trait.
use rodio::Source;
//type Sink controls  playback (represents audio track).
use rodio::Sink;

/*to play a sound:
- Create an object that represents the streaming sound. It can be a sine wave, a buffer, a decoder.
- Choose an output with the devices
- Call play_raw(output, source).
 */
pub fn play_audio() {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open("sound.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    //rodio::play_raw(&device, source.convert_samples());

    sink.append(source);
}
