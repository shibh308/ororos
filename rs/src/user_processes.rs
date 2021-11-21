use buffer::{BUF_END, ReadBuffer, WriteBuffer};
use util::{set_page_id, write_char};

pub fn echo() {

    let in_buf_addr  = 0x2000;
    let out_buf_addr = 0x3000;
    let mut in_buf  = ReadBuffer::new(in_buf_addr);
    let mut out_buf = WriteBuffer::new(out_buf_addr);

    out_buf.pipe(&mut in_buf);
}

pub fn dummy() {
    loop{}
}