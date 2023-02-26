#!/usr/bin/env rust-script

use std::process::Command;

fn main() {
    Command::new("echo").args(["spawn() create echo command"]).
        spawn().expect("failed to start `echo`");
    println!("create echo command");
}
