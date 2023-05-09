#!/usr/bin/env rust-script
// cargo-deps: nix

use std::{ffi::c_void,
          io::Write,
          num::NonZeroUsize,
          os::raw::c_int,
};

use nix::{libc::{_exit, size_t},
          sys::{
              mman::{MapFlags, mmap, ProtFlags},
              wait::waitpid,
          },
          unistd::{fork, ForkResult, getpid},
};

const ALLOC_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024_usize.pow(3)) };
const PAGE_SIZE: size_t = 4096;

fn access(mut data: *mut c_void) {
    for _ in (0..ALLOC_SIZE.get()).step_by(PAGE_SIZE) {
        unsafe {
            let target_address = data as *mut c_int;
            *(target_address) = 0;
            data = data.add(PAGE_SIZE);
        }
    }
}

fn show_meminfo(msg: &str, process: &str) {
    println!("{}", msg);
    println!("freeコマンドの実行結果：");
    let commmand = std::process::Command::new("free")
        .output()
        .expect("free failed");
    std::io::stdout()
        .write_all(&commmand.stdout)
        .expect("write failed");
    println!("{}のメモリ関連情報：", process);
    let command = std::process::Command::new("ps")
        .arg("-orss,maj_flt,min_flt")
        .arg(getpid().to_string())
        .output()
        .expect("ps failed");
    std::io::stdout()
        .write_all(&command.stdout)
        .expect("write failed");
}

fn main() {
    let addr = NonZeroUsize::new(0);

    let data = unsafe {
        mmap(
            addr,
            ALLOC_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_ANON | MapFlags::MAP_PRIVATE,
            -1,
            0,
        )
            .expect("mmap failed")
    };
    access(data);
    show_meminfo("*** 子プロセス生成前 ***", "親プロセス");
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            show_meminfo("*** 子プロセス生成直後 ***", "子プロセス");
            access(data);
            show_meminfo("*** 子プロセスによるメモリアクセス後 ***", "子プロセス");
            unsafe { _exit(0) };
        }
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).expect("waitpid failed");
        }
        Err(_) => unsafe { _exit(1) },
    }
}
