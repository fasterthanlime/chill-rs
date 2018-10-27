extern crate chill;
extern crate reqwest;

use chill::{Discard, LimitReader};
use std::io::copy;
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

    let limit = 16 * 1024;

    {
        println!("{} (1) streaming {} bytes of body...", now(), limit);
        let mut sink = Discard::new();
        let mut lr = LimitReader::new(&mut res, limit);
        let copied = copy(&mut lr, &mut sink).expect("error while streaming!");
        println!("{} (1) done streaming {} bytes", now(), copied);
    }

    {
        println!("{} (2) streaming {} bytes of body...", now(), limit);
        let mut sink = Discard::new();
        let mut lr = LimitReader::new(&mut res, limit);
        let copied = copy(&mut lr, &mut sink).expect("error while streaming!");
        println!("{} (2) done streaming {} bytes", now(), copied);
    }

    Ok(())
}
