use util::{set_clk_flg, set_page_id, write_char};

#[derive(Copy, Clone)]
pub struct Process {
    page_id: usize,
    fn_ptr: fn(),
    pc: usize,
    // TODO: ここはu32にするべきなんだけど, u32にすると動作がバグって死ぬ
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

        unsafe {
            asm!(
                "csrrsi a0,0x80,0",
                "csrrsi a1,0x83,0",
                "csrrsi sp,0x81,0",
                "jr a1"
            );
        }
    }
    pub fn copy_from_csr(&mut self) {
        // csrにある値を32個regに移動させる
        unsafe {
            asm!(
                "csrrwi {0},32,0",
                "csrrwi {1},33,0",
                "csrrwi {2},34,0",
                "csrrwi {3},35,0",
                "csrrwi {4},36,0",
                "csrrwi {5},37,0",
                "csrrwi {6},38,0",
                "csrrwi {7},39,0",
                "csrrwi {8},40,0",
                "csrrwi {9},41,0",
                out(reg) self.reg[0],
                out(reg) self.reg[1],
                out(reg) self.reg[2],
                out(reg) self.reg[3],
                out(reg) self.reg[4],
                out(reg) self.reg[5],
                out(reg) self.reg[6],
                out(reg) self.reg[7],
                out(reg) self.reg[8],
                out(reg) self.reg[9],
            );
            asm!(
                "csrrwi {0},42,0",
                "csrrwi {1},43,0",
                "csrrwi {2},44,0",
                "csrrwi {3},45,0",
                "csrrwi {4},46,0",
                "csrrwi {5},47,0",
                "csrrwi {6},48,0",
                "csrrwi {7},49,0",
                "csrrwi {8},50,0",
                "csrrwi {9},51,0",
                out(reg) self.reg[10],
                out(reg) self.reg[11],
                out(reg) self.reg[12],
                out(reg) self.reg[13],
                out(reg) self.reg[14],
                out(reg) self.reg[15],
                out(reg) self.reg[16],
                out(reg) self.reg[17],
                out(reg) self.reg[18],
                out(reg) self.reg[19],
            );
            asm!(
                "csrrwi {0},52,0",
                "csrrwi {1},53,0",
                "csrrwi {2},54,0",
                "csrrwi {3},55,0",
                "csrrwi {4},56,0",
                "csrrwi {5},57,0",
                "csrrwi {6},58,0",
                "csrrwi {7},59,0",
                "csrrwi {8},60,0",
                "csrrwi {9},61,0",
                out(reg) self.reg[20],
                out(reg) self.reg[21],
                out(reg) self.reg[22],
                out(reg) self.reg[23],
                out(reg) self.reg[24],
                out(reg) self.reg[25],
                out(reg) self.reg[26],
                out(reg) self.reg[27],
                out(reg) self.reg[28],
                out(reg) self.reg[29],
            );
            asm!(
                "csrrwi {0},62,0",
                "csrrwi {1},63,0",
                out(reg) self.reg[30],
                out(reg) self.reg[31],
            );
            // pcもここで移動
            asm!(
                "csrrwi {},0x40,0",
                out(reg) self.pc
            );
        }
    }
    pub fn copy_to_csr(&mut self) {
        // regにある値を32個csrに移動させる
        unsafe {
            asm!(
                "csrrw zero,32,{0}",
                "csrrw zero,33,{1}",
                "csrrw zero,34,{2}",
                "csrrw zero,35,{3}",
                "csrrw zero,36,{4}",
                "csrrw zero,37,{5}",
                "csrrw zero,38,{6}",
                "csrrw zero,39,{7}",
                "csrrw zero,40,{8}",
                "csrrw zero,41,{9}",
                in(reg) self.reg[0],
                in(reg) self.reg[1],
                in(reg) self.reg[2],
                in(reg) self.reg[3],
                in(reg) self.reg[4],
                in(reg) self.reg[5],
                in(reg) self.reg[6],
                in(reg) self.reg[7],
                in(reg) self.reg[8],
                in(reg) self.reg[9],
            );
            asm!(
                "csrrw zero,42,{0}",
                "csrrw zero,43,{1}",
                "csrrw zero,44,{2}",
                "csrrw zero,45,{3}",
                "csrrw zero,46,{4}",
                "csrrw zero,47,{5}",
                "csrrw zero,48,{6}",
                "csrrw zero,49,{7}",
                "csrrw zero,50,{8}",
                "csrrw zero,51,{9}",
                in(reg) self.reg[10],
                in(reg) self.reg[11],
                in(reg) self.reg[12],
                in(reg) self.reg[13],
                in(reg) self.reg[14],
                in(reg) self.reg[15],
                in(reg) self.reg[16],
                in(reg) self.reg[17],
                in(reg) self.reg[18],
                in(reg) self.reg[19],
            );
            asm!(
                "csrrw zero,52,{0}",
                "csrrw zero,53,{1}",
                "csrrw zero,54,{2}",
                "csrrw zero,55,{3}",
                "csrrw zero,56,{4}",
                "csrrw zero,57,{5}",
                "csrrw zero,58,{6}",
                "csrrw zero,59,{7}",
                "csrrw zero,60,{8}",
                "csrrw zero,61,{9}",
                in(reg) self.reg[20],
                in(reg) self.reg[21],
                in(reg) self.reg[22],
                in(reg) self.reg[23],
                in(reg) self.reg[24],
                in(reg) self.reg[25],
                in(reg) self.reg[26],
                in(reg) self.reg[27],
                in(reg) self.reg[28],
                in(reg) self.reg[29],
            );
            asm!(
                "csrrw zero,62,{0}",
                "csrrw zero,63,{1}",
                in(reg) self.reg[30],
                in(reg) self.reg[31],
            );
            // pcもここで移動
            asm!(
                "csrrw zero,0x40,{}",
                in(reg) self.pc
            );
        }
    }
}