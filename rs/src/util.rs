
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

pub fn set_page_id(page_id: usize) {
    if page_id == 0 {
        unsafe { asm!("csrrwi zero,0,0") }
    } else {
        unsafe {
            asm!("csrrwi zero,0,1",
                 "csrrw zero,1,a0")
        }
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