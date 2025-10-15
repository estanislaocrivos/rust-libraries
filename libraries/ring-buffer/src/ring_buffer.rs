pub struct RingBuffer<'a> {
    buffer: &'a mut [u8],
    size: usize,
    head: usize,
    tail: usize,
    overwrite: bool,
    was_initialized: bool,
}

impl<'a> RingBuffer<'a> {
    fn new(buffer: &'a mut [u8], size: usize, overwrite: bool) -> RingBuffer<'a> {
        return RingBuffer {
            buffer,
            size,
            head: 0,
            tail: 0,
            overwrite,
            was_initialized: true,
        };
    }
    fn push(&mut self, data: &[u8], data_size: usize) {
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
}
