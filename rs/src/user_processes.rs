use buffer::{BUF_END, ReadBuffer, WriteBuffer};
use util::{set_page_id, write_char};

pub fn echo() {
    const IN_BUF_ADDR: usize  = 0x2000;
    const OUT_BUF_ADDR: usize = 0x3000;

    let mut in_buf  = ReadBuffer::new(IN_BUF_ADDR);
    let mut out_buf = WriteBuffer::new(OUT_BUF_ADDR);

    out_buf.pipe(&mut in_buf);
}

pub fn dummy() {
    loop{}
}