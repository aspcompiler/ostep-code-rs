use std::boxed::Box;

fn main() {
    println!("location of code : {:p}", main as *const ());
    let x = Box::new(5);
    println!("location of heap : {:p}", x);
    let y = 3;
    println!("location of stack : {:p}", &y);
}
