use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).expect("Please provide a first argument");
    let filename = args.get(2).expect("Please provide a second argument");

    let contents = read_file_contents(filename);
    println!("With text:\n{}", contents);
}

fn read_file_contents(filename: &str) -> String {
    println!("In file: {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
