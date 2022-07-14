// Reproducing in Rust https://github.com/remzi-arpacidusseau/ostep-code/blob/master/intro/cpu.c
// The main difference is Rust gets arguments from env and use process::exit() for exit code.
// It is possible to use thread::sleep() but I reproduced the spin() in the original code.

mod common;

use std::{env, process};

use common::spin;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cpu <string>");
        process::exit(1);
    }
    let str = &args[1];

    loop {
        println!("{str}");
        spin(1);
    }
}
