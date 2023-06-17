#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::{
    sys::{
        mman::{mmap, MapFlags, ProtFlags},
        wait::wait,
    },
    unistd::{fork, ForkResult},
};

use std::{ffi::c_void,
          num::NonZeroUsize,
          process::exit,
};

fn main() {
    let mut data: i64 = 1000;
    println!("子プロセス生成前のデータの値：{}", data);

    let addr = NonZeroUsize::new(0);
    const PAGE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024_usize * 3) };
    let shared_memory: *mut c_void = unsafe {
        mmap(
            addr,
            PAGE_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_ANON | MapFlags::MAP_PRIVATE,
            -1,
            0,
        )
            .expect("mmap failed")
    };

    for (i, byte) in data.to_le_bytes().into_iter().enumerate() {
        unsafe {
            let addr = shared_memory.add(i) as *mut u8;
            addr.write(byte);
        }
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            wait().expect("wait failed");
            let addr = shared_memory as *mut [u8; 8];
            data = unsafe { i64::from_le_bytes(*addr) };
        }
        Ok(ForkResult::Child) => {
            let addr = shared_memory as *mut [u8; 8];
            data = unsafe { i64::from_le_bytes(*addr) };
            data *= 2;
            for (i, byte) in data.to_le_bytes().into_iter().enumerate() {
                unsafe {
                    let addr: *mut u8 = shared_memory.add(i) as *mut u8;
                    addr.write(byte);
                }
            }
            exit(0)
        }
        Err(_) => {
            exit(1)
        }
    }
    println!("子プロセス終了後のデータの値：{}", data);
}
