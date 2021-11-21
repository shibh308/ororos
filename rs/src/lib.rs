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
    }

    fn main(&mut self) {
        self.exec("test_process");
    }
}

/*

   0:	fe010113          	addi	sp,sp,-32
   4:	00112e23          	sw	ra,28(sp)
   8:	00a12623          	sw	a0,12(sp)

  clk:
   c:	00000513          	li	a0,0
  10:	00000097          	auipc	ra,0x0
  14:	000080e7          	jalr	ra # 10 <task_finished+0x10>
  18:	0040006f          	j	1c <task_finished+0x1c>

  page_id:
  1c:	00000513          	li	a0,0
  20:	00000097          	auipc	ra,0x0
  24:	000080e7          	jalr	ra # 20 <task_finished+0x20>
  28:	00a12423          	sw	a0,8(sp)
  2c:	0040006f          	j	30 <task_finished+0x30>

  os.paging.unassign ← ここヤバくて, 8(sp)は未定義なはず
  30:	00812583          	lw	a1,8(sp)
  34:	00c12503          	lw	a0,12(sp)
  38:	00000097          	auipc	ra,0x0
  3c:	000080e7          	jalr	ra # 38 <task_finished+0x38>
  40:	0040006f          	j	44 <task_finished+0x44>

  unsafe asm ← 位置関係おかしくない！？
  44:	00812503          	lw	a0,8(sp)
  48:	01051073          	csrw	0x10,a0

*/

#[no_mangle]
fn task_finished(os: &mut OS) {
    set_clk_flg(false);

    let page_id = set_page_id(0);

    // OSをCSRからメモリに再度読み込む
    let os: &mut OS;
    unsafe {
        let mut os_ptr: *mut OS;
        asm!("csrrsi {},0x80,0", out(reg) os_ptr);
        os = &mut (*os_ptr);
    }

    os.paging.unassign(page_id);

    let out_buf_addr = 0x203000 | (page_id << 16);
    let mut out_buf = ReadBuffer::new(out_buf_addr);

    out_buf.output_all();


    interrupt(os);
}

#[no_mangle]
fn interrupt(os: &mut OS) {

    set_clk_flg(false);

    // csrにレジスタ退避
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

    let prev_page_id = set_page_id(0);

    let os: &mut OS;
    unsafe {
        let mut os_ptr: *mut OS;
        asm!("csrrsi {},0x80,0", out(reg) os_ptr);
        os = &mut (*os_ptr);
    }

    if prev_page_id != 0 && os.paging.exists(prev_page_id) {
        os.paging.copy_from_csr(prev_page_id);
    }

    let page_id = os.paging.get_next_process(prev_page_id);
    write_char(page_id as u8 as char);

    if page_id == 0 {
        abort(); // 生きてるタスクがなくなったので終了
    }

    os.paging.copy_to_csr(page_id);

    // csrからレジスタ読み込み
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
    set_clk_flg(true);
    unsafe {
        // csrから読んだpcに飛ぶ
        asm!(
            "csrrwi a1,0x40,0",
            "jr a1"
        );
    }
}


#[no_mangle]
fn __start_rust() {
    let mut os = OS::new();
    unsafe {
        let os_ptr = &os as *const _ as *const u32;

        asm!(
            "csrrw zero,0x80,{0}",
            "csrrw zero,0x81,sp",
            "csrrw zero,0x82,{1}",
            "csrrw zero,0x83,{2}",
            in(reg) os_ptr,
            in(reg) interrupt,
            in(reg) task_finished,
        )
    }
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
