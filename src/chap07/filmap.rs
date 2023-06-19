#!/usr/bin/env rust-script
// cargo-deps: nix

use std::{
    io::{self, Write},
    process::Command,
    num::NonZeroUsize,
};

use nix::{
    fcntl::{open, OFlag},
    sys::{
        mman::{MapFlags,
               mmap,
               ProtFlags},
        stat::Mode,
    },
    unistd::getpid,
};

const ALLOC_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(16_usize) };

fn main() {
    let pid = getpid();
    let addr = NonZeroUsize::new(0);

    println!("testfileのメモリマップ前のプロセスの仮想アドレス空間");

    let command = Command::new("cat").arg(format!("/proc/{}/maps", pid)).output().unwrap();
    io::stdout().write_all(&command.stdout).expect("write stdout failed");

    let fd = open("testfile", OFlag::O_RDWR, Mode::empty()).unwrap();
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
            MapFlags::MAP_SHARED,
            fd,
            // ページサイズの整数倍であること
            0)
    }.unwrap();
    println!("testfileをマップしたアドレス: {:p}", data);

    println!("testfileのメモリマップ後のプロセスの仮想アドレス空間");
    let output = Command::new("cat").arg(format!("/proc/{}/maps", pid)).output().unwrap();
    io::stdout().write_all(&output.stdout).expect("write stdout failed");

    let replace_word = b"HELLO";
    for (i, b) in replace_word.into_iter().enumerate() {
        unsafe {
            (data.add(i) as *mut u8).write(*b);
        }
    }
}
