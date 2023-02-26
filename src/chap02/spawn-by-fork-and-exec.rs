#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::unistd::{fork, ForkResult, execve};
use std::ffi::CString;

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { .. }) => {
            println!("create echo command");
        }
        Ok(ForkResult::Child) => {
            let cmd = CString::new("/bin/echo").unwrap();
            let args = [
                CString::new("echo").unwrap(),
                CString::new(format!("this process is created by fork() & exec()")).unwrap(),
            ];
            let env = CString::new("").unwrap();
            execve(&cmd, &args, &[env]).expect("execve failed");
        }
        Err(_) => panic!("Fork failed"),
    }
}
