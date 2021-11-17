#![no_main]
#![no_std]

#[no_mangle]
pub fn hoge() -> i32 {
    let v = 0;
    return v + 3;
}

#[no_mangle]
pub fn huga(x: i32) -> i32 {
    return x + 5;
}

#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
    let uart = 0x1000_0000 as *mut u8;
    let a3 = hoge();
    let a8 = huga(a3);
    
    // uartへ書き込み
    unsafe {
        *uart = a8 as u8;
    }

    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop{}
}