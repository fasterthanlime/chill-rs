extern crate chill;
extern crate reqwest;

use chill::{Discard, LimitReader};
use std::io::{copy, Read};

fn main() -> Result<(), Box<std::error::Error>> {
    let url = "http://jazzblackmusic.ice.infomaniak.ch/jazzblackmusic-high.mp3";

    let client = reqwest::Client::new();
    let req = client.get(url).header("icy-metadata", "1");
    let mut res = req.send()?;

    let audio_bytes: usize = {
        let metaint = res.headers().get("icy-metaint");
        metaint.expect("invalid icecast stream").to_str()?.parse()?
    };

    let mut sink = Discard::new();
    const META_BLOCK_SIZE: usize = 16;

    loop {
        {
            let mut lr = LimitReader::new(&mut res, audio_bytes);
            copy(&mut lr, &mut sink)?;
        }

        {
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
