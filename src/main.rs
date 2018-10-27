extern crate reqwest;

use std::io::{self, Write};
use std::time::SystemTime;

fn main() -> Result<(), reqwest::Error> {
    // let url = "https://itch.io/country";
    // let url = "http://neverssl.com/";
    // let url = "http://localhost:6666";
    let url = "http://jazzblackmusic.ice.infomaniak.ch/jazzblackmusic-high.mp3";

    let start = SystemTime::now();
    let now = || format!("{:?}\t", start.elapsed().unwrap());

    let client = reqwest::Client::new();
    println!("{} get()...", now());
    let req = client.get(url).header("icy-metadata", "1");

    println!("{} send()...", now());
    let mut res = req.send()?;

    println!("{} reading headers", now());

    let audioBytes: usize = res
        .headers()
        .get("icy-metaint")
        .expect("Invalid icecast stream")
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    println!("{} audio frames will have {} bytes", now(), audioBytes);

    println!("{} streaming body...", now());
    let mut sink = Discard { count: 0 };
    res.copy_to(&mut sink)?;
    println!("{} done streaming {} bytes", now(), sink.count());

    Ok(())
}

struct Discard {
    count: usize,
}

impl Discard {
    pub fn count(&self) -> usize {
        self.count
    }
}

impl Write for Discard {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.count += buf.len();
        if self.count > 256 * 1024 {
            println!("It's been 256KiB, goodbye!");
            return Ok(0);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
