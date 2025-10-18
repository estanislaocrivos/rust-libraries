struct RingBuffer<'a> {
    buffer: &'a mut [u8],
    size: usize,
    head: usize,
    tail: usize,
    overwrite: bool,
    was_initialized: bool,
}

impl<'a> RingBuffer<'a> {
    pub fn new(buffer: &'a mut [u8], size: usize, overwrite: bool) -> RingBuffer<'a> {
        return RingBuffer {
            buffer,
            size,
            head: 0,
            tail: 0,
            overwrite,
            was_initialized: true,
        };
    }

    pub fn push(&mut self, data: &[u8], data_size: usize) {
        if !self.was_initialized {
            return;
        }
        let mut count: usize = 0;
        while count < data_size {
            let mut next: usize = self.head + 1;
            if next == self.size {
                next = 0;
            }
            if next == self.tail {
                if !self.overwrite {
                    return;
                } else {
                    self.tail += 1;
                    if self.tail == self.size {
                        self.tail = 0;
                    }
                }
            }
            self.buffer[self.head] = data[count];
            self.head = next;
            count += 1;
        }
    }

    pub fn pop(&mut self, dest: &mut [u8], dest_size: usize) {
        if !self.was_initialized {
            return;
        }
        let mut count: usize = 0;
        while count < dest_size {
            if self.tail == self.head {
                return;
            }
            dest[count] = self.buffer[self.tail];
            self.tail += 1;
            if self.tail == self.size {
                self.tail = 0;
            }
            count += 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        if !self.was_initialized {
            return false;
        }
        return self.tail == self.head;
    }

    pub fn is_full(&self) -> bool {
        let mut next_head = self.head + 1;
        if next_head == self.size {
            next_head = 0;
        }
        return next_head == self.tail;
    }

    pub fn available(&self) -> usize {
        if !self.was_initialized {
            return 0;
        }
        let used: usize;
        if self.head >= self.tail {
            used = self.head - self.tail;
        } else {
            used = self.size - (self.tail - self.head);
        }
        return self.size - used;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ring_buffer() {
        let mut buffer = [0u8; 256]; // Array de 256 bytes inicializado en 0
        let ring = RingBuffer::new(&mut buffer, 256, false);
        assert_eq!(ring.head, 0);
        assert_eq!(ring.tail, 0);
        assert_eq!(ring.size, 256);
        assert_eq!(ring.overwrite, false);
    }

    #[test]
    fn test_fill_ring_buffer() {
        let mut buffer = [0u8; 256]; // Array de 256 bytes inicializado en 0
        let mut ring = RingBuffer::new(&mut buffer, 256, false);
        let data = b"Hello, World!";
        ring.push(data, data.len());
        assert_eq!(ring.available(), 256 - data.len());
    }
}
