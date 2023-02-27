#!/usr/bin/env rust-script
// cargo-deps: signal-hook

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use signal_hook::consts::*;

fn main() -> Result<(), Error> {
    const SIGINT_U: usize = SIGINT as usize;
    let term = Arc::new(AtomicUsize::new(0));
    signal_hook::flag::register_usize(SIGINT, Arc::clone(&term), SIGINT_U)?;
    loop {
        match term.load(Ordering::Relaxed) {
            0 => (),
            SIGINT_U => {},
            _ => unreachable!(),
        }
    }
}
