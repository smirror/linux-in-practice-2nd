#!/usr/bin/env rust-script
// cargo-deps: nix, chrono

use std::{io::{self, Read},
          num::NonZeroUsize,
          os::raw::c_int
};

use nix::{
    sys::mman::{MapFlags,
                mmap,
                ProtFlags},
    libc::size_t,
};

const ALLOC_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024_usize.pow(3) ) };
const ACCESS_UNIT: size_t = 10 * 1024 ^ 2;
const PAGE_SIZE: size_t = 4096;

fn main() {
    let addr = NonZeroUsize::new(0);

    println!("Before Get new memory map.\
    Get 100MB new memory to push Enter : ");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let mut memregion = unsafe {
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

    println!("Get new memory map.\
    Get 100MB new memory each 10MB/sec to push Enter : ");
    let _ = io::stdin().read(&mut [0u8]).expect("waiting enter failed");

    for i in (0..ALLOC_SIZE.get()).step_by(PAGE_SIZE) {
        unsafe {
            let target_address = memregion as *mut c_int;
            *(target_address) = 0;
            memregion = memregion.add(PAGE_SIZE);
        }

        if i % ACCESS_UNIT == 0 && i != 0 {
            println!(
                "Access {}: {} MiB",
                chrono::Utc::now().format("%H:%M:%S"),
                i / 1024 / 1024
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    println!(
        "{}: Access to all getting new memory fields.\
        finish to push Enter：",
        chrono::Utc::now().format("%H:%M:%S").to_string()
    );
    let _ = io::stdin().read(&mut [0u8]).expect("waiting enter failed");
}
