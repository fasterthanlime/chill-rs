extern crate reqwest;

use std::io::{self, Write};
use std::time::SystemTime;

fn main() -> Result<(), reqwest::Error> {
    // let url = "https://itch.io/country";
    // let url = "http://neverssl.com/";
    let url = "http://localhost:6666";

    let start = SystemTime::now();
    let now = || format!("{:?}\t", start.elapsed().unwrap());

    let client = reqwest::Client::new();
    println!("{} get()...", now());
    let req = client.get(url);

    println!("{} send()...", now());
    let mut res = req.send()?;

    println!("{} reading headers", now());
    if let Some(loc) = res.headers().get("content-type") {
        if let Ok(loc) = loc.to_str() {
            println!("Location: {}", loc);
        }
    }

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
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
