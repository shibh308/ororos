use util::{set_clk_flg, set_page_id, write_char};

#[derive(Copy, Clone)]
pub struct Process {
    page_id: usize,
    fn_ptr: fn(),
    pc: usize,
    reg: [u8; 32],
}

impl Process {
    pub fn new(page_id: usize, fn_ptr: fn()) -> Process {
        Process {
            page_id,
            fn_ptr,
            pc: 0,
            reg: Default::default(),
        }
    }
    pub fn run(&mut self) {

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
            asm!(
                "csrrw zero,4,{}",
                in(reg) self.fn_ptr
            )
        }

        set_page_id(self.page_id);
        unsafe {
            asm!(
                "lui    sp, %hi(4096)",
                "ori    sp, sp, %lo(4096)",
            );
        }

        let fn_ptr: fn();
        unsafe { asm!("csrrwi {},4,0", out(reg) fn_ptr); }
        set_clk_flg(true);
        fn_ptr();
        set_clk_flg(false);


        /*
            TODO:
            ここからcsrrwsでOSとtask_finishedを読み込み, そこに飛ぶ
            レジスタ退避は(この関数内では)要らなくなる
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
    }
    pub fn copy_from_csr(&mut self) {
        // TODO: csrにある値を32個regに移動させる
        // pcもここで移動
    }
    pub fn copy_to_csr(&mut self) {
        // TODO: regにある値を32個csrに移動させる
        // pcもここで移動
    }
}