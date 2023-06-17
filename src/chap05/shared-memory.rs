#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::{
    sys::wait::wait,
    unistd::{fork, ForkResult},
};

use std::{
    process::exit,
    sync::{Arc, Mutex}
};

fn main() {
    let data: i64 = 1000;
    let share_data = Arc::new(Mutex::new(data));
    println!("子プロセス生成前のデータの値：{}", share_data.lock().unwrap());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            wait().expect("wait failed");
        }
        Ok(ForkResult::Child) => {
            let mut data = share_data.lock().unwrap();
            *data *= 2;
            exit(0)
        }
        Err(_) => {
            exit(1)
        }
    }
    println!("子プロセス終了後のデータの値：{}", share_data.lock().unwrap());
}
