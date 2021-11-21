#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]

mod processes;
mod paging;
mod util;
mod buffer;

use util::{write_char, write_str, set_page_id, abort, abort_str};
use paging::Paging;
use buffer::{ReadBuffer, WriteBuffer};


struct OS {
    paging: Paging,
    now_pid: usize,
}

impl OS {

    fn new() -> OS {
        OS {
            paging: Paging::new(),
            now_pid: 1,
        }
    }

    fn exec(&mut self, s: &str) {

        let pid = self.now_pid;
        self.now_pid += 1;
        let page_id = self.paging.assign(pid);

        let in_buf_addr  = 0x212000 | (page_id << 16);
        let out_buf_addr = 0x203000 | (page_id << 16);

        let mut in_buf = WriteBuffer::new(in_buf_addr);

        // 入力
        in_buf.write(95);
        in_buf.write(93);
        in_buf.close();

        unsafe {
            asm!{
                "csrrw zero,32,x0",
                "csrrw zero,33,x1",
                "csrrw zero,34,x2",
                "csrrw zero,35,x3",
                "csrrw zero,36,x4",
                "csrrw zero,37,x5",
                "csrrw zero,38,x6",
                "csrrw zero,39,x7",
                "csrrw zero,40,x8",
                "csrrw zero,41,x9",
                "csrrw zero,42,x10",
                "csrrw zero,43,x11",
                "csrrw zero,44,x12",
                "csrrw zero,45,x13",
                "csrrw zero,46,x14",
                "csrrw zero,47,x15",
                "csrrw zero,48,x16",
                "csrrw zero,49,x17",
                "csrrw zero,50,x18",
                "csrrw zero,51,x19",
                "csrrw zero,52,x20",
                "csrrw zero,53,x21",
                "csrrw zero,54,x22",
                "csrrw zero,55,x23",
                "csrrw zero,56,x24",
                "csrrw zero,57,x25",
                "csrrw zero,58,x26",
                "csrrw zero,59,x27",
                "csrrw zero,60,x28",
                "csrrw zero,61,x29",
                "csrrw zero,62,x30",
                "csrrw zero,63,x31",
            }
        }

        set_page_id(page_id);
        unsafe {
            asm!(
                "lui    sp, %hi(4096)",
                "ori    sp, sp, %lo(4096)",
            )
        }

        processes::echo();
        /* TODO: ここなんですけど, 退避後なのでmatchに使うsが死んでそう
        match s {
            "test_process" => processes::test_process(),
            _ => {},
        };
        */

        set_page_id(0);

        unsafe {
            asm!(
                "csrrwi x0,32,0",
                "csrrwi x1,33,0",
                "csrrwi x2,34,0",
                "csrrwi x3,35,0",
                "csrrwi x4,36,0",
                "csrrwi x5,37,0",
                "csrrwi x6,38,0",
                "csrrwi x7,39,0",
                "csrrwi x8,40,0",
                "csrrwi x9,41,0",
                "csrrwi x10,42,0",
                "csrrwi x11,43,0",
                "csrrwi x12,44,0",
                "csrrwi x13,45,0",
                "csrrwi x14,46,0",
                "csrrwi x15,47,0",
                "csrrwi x16,48,0",
                "csrrwi x17,49,0",
                "csrrwi x18,50,0",
                "csrrwi x19,51,0",
                "csrrwi x20,52,0",
                "csrrwi x21,53,0",
                "csrrwi x22,54,0",
                "csrrwi x23,55,0",
                "csrrwi x24,56,0",
                "csrrwi x25,57,0",
                "csrrwi x26,58,0",
                "csrrwi x27,59,0",
                "csrrwi x28,60,0",
                "csrrwi x29,61,0",
                "csrrwi x30,62,0",
                "csrrwi x31,63,0",
            );
        }

        self.paging.unassign(pid);

        // 出力
        let mut out_buf = ReadBuffer::new(out_buf_addr);
        out_buf.output_all();
    }

    fn main(&mut self) {
        self.exec("test_process");
    }
}

#[no_mangle]
extern "C" fn __start_rust() {
    let mut os = OS::new();
    os.main();
    abort();
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    write_str("panic\n");
    unsafe {
        asm!("unimp");
    }
    loop{}
}
