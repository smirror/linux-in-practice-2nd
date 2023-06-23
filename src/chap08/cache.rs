#!/usr/bin/env rust-script
// cargo-deps: nix

use nix::sys::mman::{mmap,
                     MapFlags,
                     ProtFlags};
use std::{fs::OpenOptions,
          io::Write,
          os::raw::c_void,
          num::NonZeroUsize,
          time};

const CACHE_LINE_SIZE: usize = 64;
const NACCESS: usize = 128 * 1024 * 1024;

fn main() {
    let addr = NonZeroUsize::new(0);
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("out.txt").unwrap();

    let mut i = 2.0;
    while i <= 16.0 {
        let bufsize: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2_f64.powf(i) as usize * 1024) };
        let data: *mut c_void = unsafe {
            mmap(
                addr,
                bufsize,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED | MapFlags::MAP_ANON,
                -1,
                0)
        }.unwrap();

        let bsize = usize::from(bufsize);
        println!(
            "バッファサイズ 2^{:.2}({}) KBについてのデータを収集中...",
            i,
            bsize / 1024
        );
        let start = time::Instant::now();
        let data = data as *mut u8;
        for _ in (0..(NACCESS / (bsize / CACHE_LINE_SIZE))).into_iter() {
            for j in (0..bsize).step_by(CACHE_LINE_SIZE).into_iter() {
                unsafe { data.add(j).write(0) };
            }
        }
        let end = time::Instant::now().duration_since(start).as_nanos();
        println!("{: <5}\t{}", i, (NACCESS as f64 / end as f64));
        writeln!(&mut file, "{: <5}\t{}", i, (NACCESS as f64 / end as f64)).unwrap();
        i += 0.25;
    }
}
