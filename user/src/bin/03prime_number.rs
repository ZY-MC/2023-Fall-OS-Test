#![no_std]
#![no_main]

use core::arch::asm;
use user_lib::println;

#[no_mangle]
fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num / 2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[no_mangle]
fn main() -> i32 {
    for i in 2..=100 {
        if is_prime(i) {
            unsafe {
                asm!("nop");
            }
            println!("{}", i);
        }
    }
    0
}
