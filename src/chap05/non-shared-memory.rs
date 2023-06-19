#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::{
    libc::_exit,
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};

fn main() {
    let mut data = 1000;

    println!("子プロセス生成前のデータの値：{}", data);

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            data *= 2;
            unsafe { _exit(0) };
        }
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).expect("waitpid failed");
        }
        Err(_) => unsafe { _exit(1) },
    }

    println!("子プロセス生成後のデータの値：{}", data);
}
