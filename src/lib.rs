use std::io::{self, Write};

// Discard

pub struct Discard {
    count: usize,
}

impl Discard {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

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
