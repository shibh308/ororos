#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]

mod processes;

/*

CSRについて: 容量がたくさんあるので, 適当に定義していく
0x0000:モード (0:admin,1:user)

0x0010:UART
TODO: UARTもCSR読み出しにしたい (ハード側でCSRの値を直接調整する)

*/

fn rupt() -> i32 {
    let v = 0;
    let res = processes::test_process::exec();
    return v * res;
}

#[no_mangle]
fn write_char(c: char) {
    unsafe {
        asm!("csrrw zero,16,a0");
    }
}

// 割り込みで動く処理
#[no_mangle]
fn hoge() -> i32 {
    let v = 0;
    return v + 3;
}

#[no_mangle]
fn huga(x: i32) -> i32 {
    return x + 5;
}

#[no_mangle]
extern "C" fn __start_rust() -> ! {
    let uart = 0x1000_0000 as *mut u8;
    let a3 = hoge();
    let a8 = huga(a3);
    
    // uartへ書き込み
    unsafe {
        *uart = a8 as u8;
    }

    write_char('a');

    abort();
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
fn abort(){
    unsafe {
        asm!("unimp");
    }
}