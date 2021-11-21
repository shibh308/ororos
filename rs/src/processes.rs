use buffer::{BUF_END, ReadBuffer, WriteBuffer};
use util::{set_page_id, write_char};

pub fn echo()  {

    let in_buf_addr  = 0x2000;
    let out_buf_addr = 0x3000;
    let mut in_buf  = ReadBuffer::new(in_buf_addr);
    let mut out_buf = WriteBuffer::new(out_buf_addr);

    loop{
        let res = in_buf.read();
        if res == BUF_END {
            break;
        }
        out_buf.write(res);
    }
    out_buf.close();
}