#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, getppid, getpid}};
use std::process::exit;

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Main pid={}, forked a child pid={}", getpid(), child);
            waitpid(child,None).unwrap();
        }
        Ok(ForkResult::Child) => {
            println!("Child pid={}, PPID is {}", getpid(), getppid());
            exit(0)
        }
        Err(_) => panic!("Fork failed"),
    }

}
