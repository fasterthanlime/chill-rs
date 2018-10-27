extern crate chill;
extern crate rb;
extern crate reqwest;

use chill::{ConsumerReader, LimitReader, ProducerWriter};
use rb::{SpscRb, RB};
use std::io::{copy, Read};
use std::thread;

fn main() {
    const RB_SIZE: usize = 128 * 1024;
    let rb = SpscRb::<u8>::new(RB_SIZE);
    let (prod, cons) = (rb.producer(), rb.consumer());

    let consumer_thread = thread::spawn(move || {
        let mut reader = ConsumerReader::new(cons);
        let mut buf: [u8; 16 * 1024] = [0; 16 * 1024];

        loop {
            reader.read(&mut buf).unwrap();
            println!("Just read {} bytes of data", buf.len());
        }
    });

    let http_thread = thread::spawn(|| stream(&mut ProducerWriter::new(prod)).unwrap());
    http_thread.join().unwrap();
    consumer_thread.join().unwrap();
}

fn stream<T: std::io::Write>(sink: &mut T) -> Result<(), Box<std::error::Error>> {
    let url = "http://jazzblackmusic.ice.infomaniak.ch/jazzblackmusic-high.mp3";
    let client = reqwest::Client::new();
    let req = client.get(url).header("icy-metadata", "1");
    let mut res = req.send()?;

    let audio_bytes: usize = {
        let metaint = res.headers().get("icy-metaint");
        metaint.expect("invalid icecast stream").to_str()?.parse()?
    };

    const META_BLOCK_SIZE: usize = 16;

    loop {
        {
            println!("Reading audio...");
            let mut lr = LimitReader::new(&mut res, audio_bytes);
            copy(&mut lr, sink)?;
        }

        {
            println!("Reading metadata...");
            let mut meta_blocks_buf: [u8; 1] = [0];
            res.read(&mut meta_blocks_buf)?;
            let meta_size = meta_blocks_buf[0] as usize * META_BLOCK_SIZE;

            let mut lr = LimitReader::new(&mut res, meta_size);
            let mut meta = String::new();
            lr.read_to_string(&mut meta)?;
            decode_meta(&meta);
        }
    }
}

fn decode_meta(meta: &String) {
    let meta = meta.trim().trim_matches('\0');

    if !meta.is_empty() {
        for token in meta.split(";") {
            let token = token.trim();
            if !token.is_empty() {
                let mut parts = token.splitn(2, "=");
                let (k, v) = (parts.next().unwrap().trim(), parts.next().unwrap().trim());
                if k == "StreamTitle" {
                    let v = v.trim_matches('\'');
                    if v.trim().trim_matches('-').is_empty() {
                        continue;
                    }
                    println!("Now playing: {}", v);
                }
            }
        }
    }
}
