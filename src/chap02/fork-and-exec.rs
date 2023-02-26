#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, getppid, getpid, execve}};
use std::{process::exit, ffi::CString};

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Main pid={}, forked a child pid={}", getpid(), child);
            waitpid(child,None).unwrap();
            exit(0)
        }
        Ok(ForkResult::Child) => {
            let cmd = CString::new("/bin/echo").unwrap();
            let args = [
                CString::new("echo").unwrap(),
                CString::new(format!("pid={} からこんにちは", getpid())).unwrap(),
            ];
            let env = CString::new("").unwrap();
            println!("Child pid={}, PPID is {}", getpid(), getppid());
            execve(&cmd, &args, &[env]).expect("execve failed");
            exit(0)
        }
        Err(_) => panic!("Fork failed"),
    }

}
