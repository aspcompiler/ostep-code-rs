// Reproducing in Rust https://github.com/remzi-arpacidusseau/ostep-code/blob/master/intro/mem.c
// Rust uses Bow::new() for memory allocation.
// There is a Box::new_uninit() in nightly API that I did not use.
// It is possible to print the address without unsafe using {:p}

mod common;

use std::{boxed::Box, env, process};

use common::spin;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: mem <value>");
        process::exit(1);
    }

    let mut p = Box::new(0u64); // Allocate from the heap.
    let p = p.as_mut();
    println!("{} addr pointed to by p: {:p}", process::id(), p);
    *p = args[1].parse::<u64>().unwrap(); // assign value to addr stored in p
    loop {
        spin(1);
        *p += 1;
        println!("{} value of p: {:?}", process::id(), *p);
    }
}
