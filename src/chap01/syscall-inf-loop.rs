#!/usr/bin/env rust-script
// cargo-deps: nix = "0.26.2"

fn main() {
    loop {
        nix::unistd::getppid();
    }
}