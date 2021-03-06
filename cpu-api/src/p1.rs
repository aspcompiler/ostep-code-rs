// Rust implementation of the C code: https://github.com/remzi-arpacidusseau/ostep-code/blob/master/cpu-api/p1.c

use std::{cmp::Ordering, process};

use libc::fork;

fn main() {
    println!("hello world {}", process::id());
    let rc = unsafe { fork() };
    match rc.cmp(&0) {
        Ordering::Less => {
            // fork failed; exit
            eprintln!("fork failed");
            process::exit(1);
        }
        Ordering::Equal => {
            // child (new process)
            println!("hello, I am child {}", process::id());
        }
        Ordering::Greater => {
            // parent goes down this path (original process)
            println!("hello, I am parent of {} {}", rc, process::id());
        }
    }
}
