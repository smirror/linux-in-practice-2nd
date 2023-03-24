#!/usr/bin/env rust-script

use std::{io::{self, Write},
          process::Command,
};

fn main() {
    let size = 1_000_000_000;
    println!("メモリ獲得前");
    let output = Command::new("free").output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    let _array = vec![1000 as i64; size];

    println!("メモリ獲得後");
    let output = Command::new("free").output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
}
