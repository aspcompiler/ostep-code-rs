// Reproducing in Rust https://github.com/remzi-arpacidusseau/ostep-code/blob/master/intro/io.c
// Nothing special but Rust code is simpler in this case.

use std::fs::File;
use std::io::Write;

fn main() {
    {
        let mut f = File::create("/tmp/file").unwrap();
        let buffer = "hello world\n";
        f.write_all(buffer.as_bytes()).unwrap();
        f.sync_all().unwrap();
    } // File is closed here.
}
