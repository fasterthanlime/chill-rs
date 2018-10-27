extern crate chill;
extern crate reqwest;

use chill::{Discard, LimitReader};
use std::io::{copy, Read};
use std::time::SystemTime;

fn main() -> Result<(), reqwest::Error> {
    let url = "http://jazzblackmusic.ice.infomaniak.ch/jazzblackmusic-high.mp3";

    let start = SystemTime::now();
    let now = || format!("{:?}\t", start.elapsed().unwrap());

    let client = reqwest::Client::new();
    println!("{} get()...", now());
    let req = client.get(url).header("icy-metadata", "1");

    println!("{} send()...", now());
    let mut res = req.send()?;

    println!("{} reading headers", now());

    let audio_bytes: usize = res
        .headers()
        .get("icy-metaint")
        .expect("Invalid icecast stream")
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    println!("{} audio frames will have {} bytes", now(), audio_bytes);

    let mut sink = Discard::new();
    const META_BLOCK_SIZE: usize = 16;

    loop {
        {
            println!("{} reading {} bytes of audio data...", now(), audio_bytes);
            let mut lr = LimitReader::new(&mut res, audio_bytes);
            copy(&mut lr, &mut sink).expect("while reading block data");
        }

        {
            println!("{} reading metadata size", now());
            let mut meta_blocks_buf: [u8; 1] = [0];
            res.read(&mut meta_blocks_buf)
                .expect("while reading metadata blocks size");
            let meta_size = meta_blocks_buf[0] as usize * META_BLOCK_SIZE;
            println!("{} reading {} bytes of metadata", now(), meta_size);

            let mut lr = LimitReader::new(&mut res, meta_size);
            copy(&mut lr, &mut std::io::stdout()).expect("while reading audio data");

            println!("\n\n=============\n\n")
        }
    }

    Ok(())
}
