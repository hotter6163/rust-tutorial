extern crate rust_tutorial;

use rust_tutorial::func;

fn main() {
    println!("Hello, world!");
    println!("1 + 1 = {}", func::add_one(1));
}
