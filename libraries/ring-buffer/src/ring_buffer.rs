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

    fn pop(&mut self, dest: &mut [u8], dest_size: usize) {
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

    fn is_empty(&self) -> bool {
        if !self.was_initialized {
            return false;
        }
        return self.tail == self.head;
    }

    fn is_full(&self) -> bool {
        let mut next_head = self.head + 1;
        if next_head == self.size {
            next_head = 0;
        }
        return next_head == self.tail;
    }

    fn available(&self) -> usize {
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
