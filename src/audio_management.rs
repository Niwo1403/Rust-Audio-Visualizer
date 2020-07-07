//Source: https://docs.rs/rodio/0.11.0/rodio/

use std::io::{BufReader, Read};
//use rodio::Source;    //Rust object that represents a sound should implement the Source trait.
use rodio::{Sink, Source, Sample, Decoder};       //type Sink controls  playback (represents audio track).
use std::fs::File;
use std::time::Duration;     //to read bytes of file
use std::sync::mpsc::Sender;

/*to play a sound:
- Create an object that represents the streaming sound. It can be a sine wave, a buffer, a decoder.
- Choose an output with the devices
- Call play_raw(output, source).
 */
pub fn play_audio(arg: &str, value_sender: Sender<f32>) {
    println!("Filename: {:?}", arg);
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open(arg).unwrap();  //added
    let source = Decoder::new(BufReader::new(file)).unwrap();
    //rodio::play_raw(&device, source.convert_samples()); //

    struct WrappedSource <T> {
        source_box: std::boxed::Box<T>, //  rodio::source::SamplesConverter<rodio::Decoder<BufReader<File>>, dyn Sample>>,
        value_sender: Sender<f32>
    };
    impl <T> Iterator for WrappedSource <T> where T: Iterator<Item=f32> {
        type Item = f32;

        fn next(&mut self) -> Option<f32> {
            let elm = (*self.source_box).next();
            if let Some(elm) = elm {
                // pass f32 to transformation
                self.value_sender.send(elm);
            }
            return elm;
        }
    };
    impl <S> Source for WrappedSource <S> where S: Iterator<Item=f32> {
        fn current_frame_len(&self) -> Option<usize> {
            return self.current_frame_len();
        }

        fn channels(&self) -> u16 {
            return self.channels();
        }

        fn sample_rate(&self) -> u32 {
            return self.sample_rate();
        }

        fn total_duration(&self) -> Option<Duration> {
            return self.total_duration();
        }
    };
    //let wrapped_source:WrappedSource<rodio::source::SamplesConverter<Decoder<BufReader<File>>, f32>> = WrappedSource { source_box: Box::new(source.convert_samples()), value_sender: value_sender};


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

