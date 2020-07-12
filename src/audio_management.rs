//Source: https://docs.rs/rodio/0.11.0/rodio/

use std::io::BufReader;
//use rodio::Source;    //Rust object that represents a sound should implement the Source trait.
use rodio::{Sink, Source, Decoder};       //type Sink controls  playback (represents audio track).
use std::fs::File;
use std::time::Duration;     //to read bytes of file
use std::sync::mpsc::Sender;


pub fn play_audio(arg: &str, value_sender: Sender<f32>) {
    println!("Filename: {:?}", arg);
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open(arg).unwrap();  //added
    let source = Decoder::new(BufReader::new(file)).unwrap();

    struct WrappedSource <T> {
        source_box: std::boxed::Box<T>,
        value_sender: Sender<f32>,
        channel_count: u16,
    };
    impl <T> Iterator for WrappedSource <T> where T: Iterator<Item=f32>, T: Source {
        type Item = f32;

        fn next(&mut self) -> Option<f32> {
            let elm = (*self.source_box).next();
            if let Some(elm) = elm {

                if self.channel_count == 0 {
                    // pass f32 to transformation
                    self.value_sender.send(elm);
                }
                self.channel_count += 1;
                if self.channel_count >= (*self.source_box).channels(){
                    self.channel_count = 0;
                }
            }
            return elm;
        }
    };
    impl <S> Source for WrappedSource <S> where S: Iterator<Item=f32>, S: Source {
        fn current_frame_len(&self) -> Option<usize> {
            return (*self.source_box).current_frame_len();
        }

        fn channels(&self) -> u16 {
            return (*self.source_box).channels();
        }

        fn sample_rate(&self) -> u32 {
            return (*self.source_box).sample_rate();
        }

        fn total_duration(&self) -> Option<Duration> {
            return (*self.source_box).total_duration();
        }
    };
    let wrapped_source:WrappedSource<rodio::source::SamplesConverter<Decoder<BufReader<File>>, f32>> = WrappedSource { source_box: Box::new(source.convert_samples()), value_sender: value_sender, channel_count: 0};


    sink.append(wrapped_source);
    sink.sleep_until_end();
}
