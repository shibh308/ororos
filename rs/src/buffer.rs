
use util::write_char;

pub const BUF_END: u8 = !(0 as u8);

pub struct ReadBuffer {
    start_addr: usize,
    now: usize,
}

impl ReadBuffer {
    pub fn new(addr: usize) -> ReadBuffer {
        ReadBuffer { start_addr: addr, now: addr }
    }

    pub fn read(&mut self) -> u8 {
        unsafe {
            let ptr = self.now as *const u8;
            let res = *ptr;
            self.now += 1;
            res
        }
    }

    pub fn output_all(&mut self) {
        loop{
            let res = self.read();
            if res == BUF_END {
                break;
            }
            write_char(res as char);
        }
    }
}

pub struct WriteBuffer {
    start_addr: usize,
    now: usize,
}

impl WriteBuffer {
    pub fn new(addr: usize) -> WriteBuffer {
        WriteBuffer { start_addr: addr, now: addr }
    }

    pub fn write(&mut self, x: u8) {
        unsafe {
            let ptr = self.now as *mut u8;
            *ptr = x;
            self.now += 1;
        }
    }
    pub fn close(&mut self) {
        self.write(BUF_END);
    }
    pub fn pipe(&mut self, rb: &mut ReadBuffer) {
        loop {
            let res = rb.read();
            write_char('d'); // TODO: ここの直後でinterruptしてる
            write_char(res as char);
            if res == BUF_END {
                break;
            }
            self.write(res);
        }
        self.close();
    }
}