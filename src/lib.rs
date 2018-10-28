extern crate rb;

use rb::{Consumer, Producer, RbConsumer, RbProducer};
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};

// Discard

pub struct Discard {}

impl Discard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Write for Discard {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
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
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
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
    pos: u64,
}

impl ConsumerReader {
    pub fn new(inner: Consumer<u8>) -> Self {
        Self { inner, pos: 0 }
    }
}

impl Read for ConsumerReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner.read_blocking(buf) {
            Some(bytes) => {
                self.pos += bytes as u64;
                Ok(bytes)
            }
            None => Ok(0),
        }
    }
}

impl Seek for ConsumerReader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(x) => println!("seek Start({})", x),
            SeekFrom::End(x) => println!("seek End({})", x),
            SeekFrom::Current(x) => println!("seek Current({})", x),
        }

        Err(io::Error::new(
            ErrorKind::InvalidInput,
            "can't seek into stream",
        ))
    }
}

// Implementation copied from rodio so it doesn't try wav first.

extern crate minimp3;
extern crate rodio;

use minimp3::{Decoder, Frame};
use rodio::Source;
use std::time::Duration;

pub struct Mp3Decoder<R>
where
    R: Read + Seek,
{
    decoder: Decoder<R>,
    current_frame: Frame,
    current_frame_offset: usize,
}

impl<R> Mp3Decoder<R>
where
    R: Read + Seek,
{
    pub fn new(data: R) -> Result<Self, ()> {
        let mut decoder = Decoder::new(data);
        let current_frame = decoder.next_frame().map_err(|_| ())?;

        Ok(Mp3Decoder {
            decoder,
            current_frame,
            current_frame_offset: 0,
        })
    }
}

impl<R> Source for Mp3Decoder<R>
where
    R: Read + Seek,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.current_frame.data.len())
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.current_frame.channels as _
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.current_frame.sample_rate as _
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl<R> Iterator for Mp3Decoder<R>
where
    R: Read + Seek,
{
    type Item = i16;

    #[inline]
    fn next(&mut self) -> Option<i16> {
        if self.current_frame_offset == self.current_frame.data.len() {
            self.current_frame_offset = 0;
            match self.decoder.next_frame() {
                Ok(frame) => self.current_frame = frame,
                _ => return None,
            }
        }

        let v = self.current_frame.data[self.current_frame_offset];
        self.current_frame_offset += 1;

        return Some(v);
    }
}
