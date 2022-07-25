#![no_std] // dont't use std, use core instead
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

// 引入asm代码
use core::arch::global_asm;                 
global_asm!(include_str!("entry.asm"));     

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    })
}
