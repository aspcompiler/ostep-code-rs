// Rust implementation of the C code: https://github.com/remzi-arpacidusseau/ostep-code/blob/master/cpu-api/p2.c

use std::{cmp::Ordering, process, ptr, thread, time::Duration};

use libc::{fork, wait};

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
            thread::sleep(Duration::from_secs(1));
        }
        Ordering::Greater => {
            // parent goes down this path (original process)
            let wc = unsafe { wait(ptr::null_mut()) };
            println!("hello, I am parent of {} {} {}", rc, wc, process::id());
        }
    }
}
