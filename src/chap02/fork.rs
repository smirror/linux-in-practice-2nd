#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::unistd::{fork, ForkResult, getppid, getpid};

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Main pid={}, forked a child pid={}", getpid(), child);
        }
        Ok(ForkResult::Child) => {
            println!("Child pid={}, PPID is {}", getpid(), getppid());
        }
        Err(_) => println!("Fork failed"),
    }

}
