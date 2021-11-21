#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]

pub mod process;
mod user_processes;
mod paging;
mod util;
mod buffer;

use util::{write_char, write_str, set_page_id, set_clk_flg, abort, abort_str};
use paging::Paging;
use buffer::{ReadBuffer, WriteBuffer};


struct OS {
    paging: Paging,
}

impl OS {

    fn new() -> OS {
        OS {
            paging: Paging::new(),
        }
    }

    fn exec(&mut self, s: &str) {

        // TODO: ここなんか挙動おかしい
        let call_func = match s {
            _ => user_processes::echo,
        };

        let page_id = self.paging.assign(call_func);

        let in_buf_addr  = 0x202000 | (page_id << 16);
        let mut in_buf = WriteBuffer::new(in_buf_addr);

        // 入力
        in_buf.write(95);
        in_buf.write(93);
        in_buf.close();

        self.paging.run(page_id);

        self.paging.unassign(page_id);

        let out_buf_addr = 0x203000 | (page_id << 16);
        let mut out_buf = ReadBuffer::new(out_buf_addr);
        out_buf.output_all();
    }

    #[no_mangle]
    fn task_finished(&mut self) {
        set_clk_flg(false);
        let page_id = set_page_id(0);

        self.paging.unassign(page_id);

        let out_buf_addr = 0x203000 | (page_id << 16);
        let mut out_buf = ReadBuffer::new(out_buf_addr);
        out_buf.output_all();

        abort(); // とりあえずここで終わらせる
        self.interrupt();
    }

    #[no_mangle]
    fn interrupt(&mut self) {

        // ------------------------以下TODO-----------------------

        set_clk_flg(false);

        // TODO: ここでcsrにレジスタ退避

        let prev_page_id = set_page_id(0);
        if prev_page_id != 0 {
            self.paging.copy_from_csr(prev_page_id);
        }

        let page_id = self.paging.get_process(prev_page_id);
        if page_id == 0 {
            abort(); // 生きてるタスクがなくなったので終了
        }

        self.paging.copy_to_csr(page_id);
        /*
            TODO: ここでcsrからレジスタ読み込み
        */
        set_clk_flg(true);

        // TODO: ここでcsrから読んだpcに移動する
    }

    fn main(&mut self) {
        self.exec("test_process");
    }
}

#[no_mangle]
fn __start_rust() {
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
