This is a WIP implementation of [C code](https://github.com/remzi-arpacidusseau/ostep-code) from Operating Systems: Three Easy Pieces in Rust.

Why do I want to port them to Rust?

1. I use it as a project to practice Rust. I got the opportunity to experiment with a wide variety of APIs in an OS.
1. It allows me to compare Rust to C so I concrete information on which one is better for a particular situation.
1. I am contemplating of writing another OS in Rust.

Whenever we have a choice between unsafe `libc` or safe Rust `std`, we always go with the safe one as that is what make
Rust better. A few exceptions are:

1. intro/threads.rs: You need unsafe to demonstrate the synchronization problem. The safe Rust actually prevents such problem.
1. cpu-api.rs: We use `libc` to demonstrated `fork()`.