#![no_std] // dont't use std, use core instead
#![no_main]
mod lang_items;

// 引入asm代码
use core::arch::global_asm;                 
global_asm!(include_str!("entry.asm"));     
