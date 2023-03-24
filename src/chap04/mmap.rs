#!/usr/bin/env rust-script
// cargo-deps: nix

use std::{io::{self, Write},
          process::Command,
};
use std::num::NonZeroUsize;

use nix::{
    sys::mman::{MapFlags,
                mmap,
                ProtFlags},
    unistd::getpid,
};

fn main() {
    const ALLOC_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024 ^ 3) };

    let pid = getpid();
    let addr = NonZeroUsize::new(0);
    println!("Before Get new memory map");
    let command = Command::new("cat").arg(format!("/proc/{}/maps", pid)).output().unwrap();
    io::stdout().write_all(&command.stdout).expect("write stdout failed");

    // get !GB memory by mmap()
    let data = unsafe {
        mmap(
            // アドレス、通常は0を指定
            addr,
            // メモリにマップするサイズ
            ALLOC_SIZE,
            // メモリ保護の指定
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            // マップされたオブジェクトのタイプ、マップ時のオプション、
            // マップされたページコピーへの変更を そのプロセスだけが行えるのかを指定する
            MapFlags::MAP_ANON | MapFlags::MAP_PRIVATE,
            // MAP_ANONがセットされた場合は-1にするとだけまず覚えておく
            -1,
            // ページサイズの整数倍であること
            0)
    }.unwrap();
    println!("new memory field： address = {:p}, size = 0x{:x}", data, ALLOC_SIZE);

    println!("After Get new memory map");
    let command = Command::new("cat").arg(format!("/proc/{}/maps", pid)).output().unwrap();
    io::stdout().write_all(&command.stdout).expect("write stdout failed");
}
