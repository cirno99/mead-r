/*
MIT License

Copyright (c) 2022 Joker2770

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use clap::{App, Arg};
use morse;
use std::{thread::sleep, time::Duration};
use anyhow::Result;
use rodio::Source;

pub mod args;
use crate::args::{Message, Unit};

fn main() {
    let matches = App::new("mead-r")
        .version("0.1.0")
        .author("Jintao Yang <yjt950840@outlook.com>")
        .about("An application of morse-encode-and-decode rewrite in Rust programming language.")
        .arg(
            Arg::with_name("encode string")
                .short('e')
                .long("encode")
                .takes_value(true)
                .help("String to encode."),
        )
        .arg(
            Arg::with_name("decode string")
                .short('d')
                .long("decode")
                .takes_value(true)
                .help("String to decode."),
        )
        .arg(
            Arg::with_name("covert into sound")
                .short('c')
                .long("convert")
                .takes_value(true)
                .help("Covert into sound."),
        )
        .get_matches();
    if let Some(s) = matches.value_of("encode string") {
        use_encode(s);
    }
    if let Some(s) = matches.value_of("decode string") {
        use_decode(s);
    }
    if let Some(s) = matches.value_of("covert into sound") {
        let s = use_encode(s).expect("error");
        morse2sound(s);
    }
}

fn use_encode(s: &str) -> Option<String> {
    match morse::encode::encode(s) {
        Ok(x) => {
            println!("{}", x);
            Some(x)
    },
        Err(e) => {
            println!(
                "The following chars were unsupported {:?}",
                e.unsupported_characters
            );
            None
        }
    }
}

fn use_decode(s: &str) {
    match morse::decode::decode(s) {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!(
                "The following chars were unsupported {:?}",
                e.unsupported_characters
            )
        }
    }
}

fn morse2sound(message: String) -> Result<()> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;

    let tone = rodio::source::SineWave::new(900);
    let dot_duration = Duration::from_millis(80);
    let dot = tone.clone().take_duration(dot_duration);
    let dash = tone.take_duration(dot_duration * 3);
    let message = message.parse::<Message>()?;
    for unit in message.0 {
        match unit {
            Unit::Dot => {
                sink.append(dot.clone());
                sink.sleep_until_end();
                sleep(dot_duration);
            }
            Unit::Dash => {
                sink.append(dash.clone());
                sink.sleep_until_end();
                sleep(dot_duration)
            }
            Unit::Space => {
                sleep(dot_duration * 3);
            }
            Unit::Slash => {
                sleep(dot_duration * 3);
            }
        }
    }
    Ok(())
}
