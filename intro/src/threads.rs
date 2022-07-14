// Reproducing in Rust https://github.com/remzi-arpacidusseau/ostep-code/blob/master/intro/threads.c
// Use `static` to create global variables.
// static mut is unsafe.

use std::{env, process, thread};

static mut COUNTER: u64 = 0;
static mut LOOPS: u64 = 0;

fn worker() {
    unsafe {
        for _ in 0..LOOPS {
            COUNTER += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: threads <loops>");
        process::exit(1);
    }
    unsafe {
        LOOPS = args[1].parse::<u64>().unwrap();
        println!("Initial value : {}", COUNTER);
    }
    let p1 = thread::spawn(worker);
    let p2 = thread::spawn(worker);
    p1.join().unwrap();
    p2.join().unwrap();
    unsafe {
        println!("Final value   : {}", COUNTER);
    }
}
