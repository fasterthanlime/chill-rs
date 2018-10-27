extern crate rb;

use rb::{Consumer, Producer, RbConsumer, RbProducer};
use std::io::{self, Read, Write};

// Discard

pub struct Discard {}

impl Discard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Write for Discard {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

// Limited reader

pub struct LimitReader<'a, T: Read + 'static> {
    inner: &'a mut T,
    remaining: usize,
}

impl<'a, T: Read> LimitReader<'a, T> {
    pub fn new(inner: &'a mut T, remaining: usize) -> Self {
        Self { inner, remaining }
    }
}

impl<'a, T: Read> Read for LimitReader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if self.remaining == 0 {
            // indicate EOF
            return Ok(0);
        }

        let res = if self.remaining < buf.len() {
            // last buffer might be smaller, due to our limit
            self.inner.read(&mut buf[..self.remaining])
        } else {
            self.inner.read(buf)
        };

        if let Ok(size) = res {
            self.remaining -= size;
        }
        res
    }
}

//

pub struct ProducerWriter {
    inner: Producer<u8>,
}

impl ProducerWriter {
    pub fn new(inner: Producer<u8>) -> Self {
        Self { inner }
    }
}

impl Write for ProducerWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.inner.write_blocking(buf) {
            Some(bytes) => Ok(bytes),
            None => Ok(0),
        }
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

//

pub struct ConsumerReader {
    inner: Consumer<u8>,
}

impl ConsumerReader {
    pub fn new(inner: Consumer<u8>) -> Self {
        Self { inner }
    }
}

impl Read for ConsumerReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self.inner.read_blocking(buf) {
            Some(bytes) => Ok(bytes),
            None => Ok(0),
        }
    }
}
