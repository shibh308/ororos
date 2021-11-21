
pub fn write_char(_c: char) {
    unsafe {
        asm!("csrrw zero,16,a0");
    }
}

pub fn write_str(s: &str) {
    for c in s.chars() {
        write_char(c)
    }
}

pub fn set_clk_flg(fl: bool) {
    let fl = fl as u8;
    unsafe { asm!("csrrw zero,2,{}",
                  "csrrwi zero,3,0",
                   in(reg) fl) }
}

pub fn set_page_id(page_id: usize) -> usize {
    unsafe {
        asm!("csrrsi a6,0,0");
        if page_id == 0 {
            asm!("csrrwi zero,0,0")
        } else {
            asm!("csrrwi zero,0,1",
                 "csrrw zero,1,a0")
        }
        let res;
        asm!("mv {},a6", out(reg) res);
        res
    }
}

pub fn abort_str(s: &str){
    write_str(s);
    abort();
}

pub fn abort(){
    unsafe {
        asm!("unimp");
    }
}