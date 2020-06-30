//Source: https://docs.rs/rodio/0.11.0/rodio/

use std::io::{BufReader, Read};
//use rodio::Source;    //Rust object that represents a sound should implement the Source trait.
use rodio::{Sink, Source, Sample};       //type Sink controls  playback (represents audio track).
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

    struct WrappedSource {
        source_box: std::boxed::Box<rodio::source::SamplesConverter<rodio::Decoder<BufReader<File>>, Sample>>,
    };
    impl Iterator for WrappedSource {
        type Item = Option<I::Item>;

        fn next(&mut self) -> Option<f32::Item> {
            let elm = self.source_box.next();
            if let Some(elm) = elm {
                // do sth with elm...
                // visualtize(elm);
            }
            return elm;
        }
    };
    let wrapped_source = WrappedSource { source_box: Box::new(source.convert_samples())};

    sink.append(source);
}

/*
pub fn read_file(arg: &str) -> Vec<i8> {
    let mut f = File::open(arg)?;
    let mut buffer: Vec<u8> = Vec::new();
    // Read all bytes until EOF in this source, placing them into buf.
    // All bytes read from this source will be appended to the specified buffer buf.This function will continuously call read() to append more data to buf until read() returns either Ok(0) or an error of non-ErrorKind::Interrupted kind.
    // If successful, this function will return the total number of bytes read.
    let mut read_into: Vec<i8> = vec![0; 128];
    f.read_i8_into(&mut read_into).unwrap(); // read_to_end(&mut buffer)?;
    return read_into;
}
*/

