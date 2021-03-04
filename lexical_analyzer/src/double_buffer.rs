//! A specialized buffer for lexical analysis with a constant space complexity
//! The size of the buffer limits the length of any analysis of the file and
//! can be changed by modifying BUFFER_SIZE
use log::{error, trace};
use std::io::Read;
use std::ops::Index;

pub const BUFFER_SIZE: usize = 4096; // this value should not exceed half the stack size
pub const DBUF_COUNT: usize = 2;

pub struct DoubleFixedBuffer<T: Read> {
    data: T,
    buffers: [[u8; BUFFER_SIZE]; DBUF_COUNT],
    buffer_sizes: [usize; DBUF_COUNT], // Actual amount of valid data in the buffer
    last_loaded: u8,                   // Which buffer was most recently read from the file
    end_of_input: bool,
}

impl<T: Read> DoubleFixedBuffer<T> {
    pub fn new(data: T) -> Self {
        let mut result = DoubleFixedBuffer {
            data,
            buffers: [[0; BUFFER_SIZE]; DBUF_COUNT],
            buffer_sizes: [0, 0],
            last_loaded: (DBUF_COUNT - 1) as u8,
            end_of_input: false,
        };
        if result.read_next() == 0 {
            result.last_loaded = 0;
            result.end_of_input = true;
        }
        result.read_next();
        result
    }

    /// Read up to BUFFER_SIZE bytes into the buffer that wasn't most recently filled
    /// Ex. If buffer 1 was last filled by a read_next(), the subsequent call will fill
    /// buffer 2
    fn read_next(&mut self) -> usize {
        if self.end_of_input {
            return 0;
        }

        // The nature of a read error on an opened file usually indicates something unrecoverable
        // such as the file being deleted or corrupted
        let mut temp = [0u8; BUFFER_SIZE];
        let bytes_read = match self.data.read(&mut temp) {
            Ok(n) => n,
            Err(err) => {
                error!(
                    "Failed to read next block of {} bytes from file: {}",
                    BUFFER_SIZE, err
                );
                panic!();
            }
        };

        if bytes_read == 0 {
            self.end_of_input = true
        } else {
            self.last_loaded = (self.last_loaded + 1) % DBUF_COUNT as u8;
            self.buffers[self.last_loaded as usize] = temp;
            self.buffer_sizes[self.last_loaded as usize] = bytes_read;
            trace!("Now active: {}", self.last_loaded + 1);
            trace!(
                "Read {} bytes into buffer {}",
                bytes_read,
                self.last_loaded + 1
            );
            trace!("B1 {:?}", self.buffers[0]);
            trace!("B2 {:?}", self.buffers[1]);
        }
        bytes_read
    }

    pub fn end_of_input(&self) -> DoubleFixedBufferCursor {
        if self.end_of_input {
            DoubleFixedBufferCursor {
                buffer_index: self.last_loaded,
                buffer_position: self.buffer_sizes[self.last_loaded as usize],
            }
        } else {
            DoubleFixedBufferCursor {
                buffer_index: u8::MAX,
                buffer_position: usize::MAX,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoubleFixedBufferCursor {
    buffer_index: u8,
    buffer_position: usize,
}

impl DoubleFixedBufferCursor {
    pub fn new() -> Self {
        DoubleFixedBufferCursor {
            buffer_index: 0,
            buffer_position: 0,
        }
    }

    pub fn advance<T: Read>(&mut self, buffer: &mut DoubleFixedBuffer<T>) {
        trace!("Advancing {}", self);
        if *self == buffer.end_of_input() {
            return;
        }

        self.buffer_position += 1;
        if self.end_of_buffer(buffer) {
            if self.in_last_loaded_buffer(buffer) {
                buffer.read_next(); // read_next changed the value of end_of_input
            }

            // Check it before advancing into bad memory
            if *self != buffer.end_of_input() {
                self.buffer_position = 0;
                self.buffer_index = (self.buffer_index + 1) % DBUF_COUNT as u8;
            }
        }
    }

    pub fn readonly_advance<T: Read>(&mut self, buffer: &DoubleFixedBuffer<T>) {
        trace!("RO Advancing {}", self);
        if *self == buffer.end_of_input() {
            error!("Read-only cursor advancing past end of input");
            panic!();
        }

        self.buffer_position += 1;
        if self.end_of_buffer(buffer) {
            self.buffer_position = 0;
            self.buffer_index = (self.buffer_index + 1) % DBUF_COUNT as u8;
        }
    }

    pub fn copy_bytes<T: Read>(&self, end: usize, buffer: &DoubleFixedBuffer<T>) -> Vec<u8> {
        trace!("Copying from {}, {} bytes", self, end);
        let mut result = Vec::new();
        let mut position = *self;
        for _ in 0..end {
            trace!("Copying position {} into buffer", position);
            result.push(buffer[position]);
            position.readonly_advance(buffer);
        }
        result
    }

    pub fn in_last_loaded_buffer<T: Read>(&self, buffer: &DoubleFixedBuffer<T>) -> bool {
        trace!(
            "Checking if {} is in the last loaded buffer: last loaded {}",
            self,
            buffer.last_loaded
        );
        self.buffer_index == buffer.last_loaded
    }

    pub fn end_of_buffer<T: Read>(&self, buffer: &DoubleFixedBuffer<T>) -> bool {
        trace!(
            "Checking if {} is past end of buffer: size {}",
            self,
            buffer.buffer_sizes[self.buffer_index as usize]
        );
        self.buffer_position == buffer.buffer_sizes[self.buffer_index as usize]
    }
}

impl<T: Read> Index<DoubleFixedBufferCursor> for DoubleFixedBuffer<T> {
    type Output = u8;

    fn index(&self, cursor: DoubleFixedBufferCursor) -> &Self::Output {
        &self.buffers[cursor.buffer_index as usize][cursor.buffer_position]
    }
}

impl std::fmt::Display for DoubleFixedBufferCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(buffer: {}, pos: {})",
            self.buffer_index, self.buffer_position
        )
    }
}
