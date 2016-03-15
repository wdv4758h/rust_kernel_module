#![feature(core_str_ext)]
#![no_std]

mod kernel;

#[macro_use]
mod utils;


////////////////////////////////////////
// Kernel Module's basic functions
////////////////////////////////////////

#[no_mangle]
pub fn rust_init() -> i32 {
    println!("rust: Hello");
    0
}

#[no_mangle]
pub fn rust_exit() {
    println!("rust: Goodbye");
}
