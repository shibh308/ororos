
use util::write_char;

pub const BUF_END: u8 = !(0 as u8);

pub struct ReadBuffer {
    addr: usize
}

impl ReadBuffer {
    pub fn new(addr: usize) -> ReadBuffer {
        ReadBuffer { addr }
    }

    pub fn read(&mut self) -> u8 {
        unsafe {
            let ptr = self.addr as *const u8;
            let res = *ptr;
            self.addr += 1;
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
    addr: usize
}

impl WriteBuffer {
    pub fn new(addr: usize) -> WriteBuffer {
        WriteBuffer { addr }
    }

    pub fn write(&mut self, x: u8) {
        unsafe {
            let ptr = self.addr as *mut u8;
            *ptr = x;
            self.addr += 1;
        }
    }
    pub fn close(&mut self) {
        self.write(BUF_END);
    }
}