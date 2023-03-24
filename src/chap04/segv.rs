#!/usr/bin/env rust-script

fn main() {
    println!("Before access invalid memory address");
    unsafe { std::ptr::null_mut::<i32>().write(42) };
    println!("After access invalid memory address");
}
