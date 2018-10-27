extern crate chill;
extern crate reqwest;

use chill::{Discard, LimitReader};
use std::io::{copy, Read};
use std::str::Split;
use std::time::SystemTime;

fn main() -> Result<(), Box<std::error::Error>> {
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
        .to_str()?
        .parse()?;
    println!("{} audio frames will have {} bytes", now(), audio_bytes);

    let mut sink = Discard::new();
    const META_BLOCK_SIZE: usize = 16;

    loop {
        {
            // println!("{} reading {} bytes of audio data...", now(), audio_bytes);
            let mut lr = LimitReader::new(&mut res, audio_bytes);
            copy(&mut lr, &mut sink)?;
        }

        {
            // println!("{} reading metadata size", now());
            let mut meta_blocks_buf: [u8; 1] = [0];
            res.read(&mut meta_blocks_buf)?;
            let meta_size = meta_blocks_buf[0] as usize * META_BLOCK_SIZE;
            // println!("{} reading {} bytes of metadata", now(), meta_size);

            let mut lr = LimitReader::new(&mut res, meta_size);
            let mut meta = String::new();
            lr.read_to_string(&mut meta)?;
            let meta = meta.trim().trim_matches('\0');

            if !meta.is_empty() {
                for token in meta.split(";") {
                    let token = token.trim();
                    if !token.is_empty() {
                        let mut parts = token.splitn(2, "=");
                        let (k, v) = (parts.next().unwrap().trim(), parts.next().unwrap().trim());
                        if k == "StreamTitle" {
                            let v = v.trim_matches('\'');
                            println!("Now playing: {}", v);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
