extern crate nix;

use nix::unistd::{getpid};

fn main() {
    loop {
        getpid();
    }
}
