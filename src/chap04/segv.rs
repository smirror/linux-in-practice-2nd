#!/usr/bin/env rust-script
// cargo-deps: libc

use libc::kill;
use std::process;

fn main() {
    unsafe {
        // First SIGSEGV will be consumed by Rust runtime
        // (see https://users.rust-lang.org/t/is-sigsegv-handled-by-rust-runtime/45680)...
        println!("Before access invalid memory address");
        kill(process::id() as i32, libc::SIGSEGV);
        // ...but the second will crash the program, as expected
        kill(process::id() as i32, libc::SIGSEGV);
        println!("After access invalid memory address");
    }
}
