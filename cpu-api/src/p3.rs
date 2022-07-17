// Rust implementation of the C code: https://github.com/remzi-arpacidusseau/ostep-code/blob/master/cpu-api/p3.c
// Convert Rust string to C *const c_char is ugly. We used a utility to do that.

use std::{cmp::Ordering, ffi::CString, process, ptr};

use libc::{c_char, execvp, fork, wait};

// Adapted from https://gist.github.com/TrinityCoder/793c097b5a4ab25b8fabf5cd67e92f05
/// Consumes a `Vec<String>` and saves some helper data, so it can provide
/// the `*const *const c_char` (`argv`) and `*const c_char` (`argv[0]`)
/// when the user needs them i. e. for [`libc::execvp()`] library call.
pub struct Argv {
    _argv: Vec<CString>, // Use it to hold the lifetime of the `CString`s.
    argv_ptr: Vec<*const c_char>,
}

impl Argv {
    /// Creates a new `Argv` structure.
    pub fn new(args: Vec<String>) -> Self {
        let argv: Vec<_> = args
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
        let mut argv_ptr: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();
        argv_ptr.push(std::ptr::null());
        Self {
            _argv: argv,
            argv_ptr,
        }
    }

    /// Returns the C language's `argv` (`*const *const c_char`).
    pub fn get_argv(&self) -> *const *const c_char {
        self.argv_ptr.as_ptr()
    }

    /// Returns the C language's `argv[0]` (`*const c_char`).
    pub fn get_argv0(&self) -> *const c_char {
        self.argv_ptr[0]
    }

    /// Gets total length of the `argv` array (including the last null pointer).
    pub fn get_len(&self) -> usize {
        self.argv_ptr.len()
    }
}

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
            // program: "wc" (word count), argument: file to count
            let myargs = vec!["wc".to_string(), "src/p3.rs".to_string()];
            let argv = Argv::new(myargs);
            unsafe { execvp(argv.get_argv0(), argv.get_argv()) };
            println!("this shouldn't print out");
        }
        Ordering::Greater => {
            // parent goes down this path (original process)
            let wc = unsafe { wait(ptr::null_mut()) };
            println!("hello, I am parent of {} {} {}", rc, wc, process::id());
        }
    }
}
