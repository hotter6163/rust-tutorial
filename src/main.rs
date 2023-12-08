// extern crate rust_tutorial;

// use std::env;
// use std::process;

// use rust_tutorial::Config;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     if let Err(e) = rust_tutorial::run(config) {
//         eprintln!("Application error: {}", e);
//         process::exit(1);
//     }
// }

// --------------------

fn main() {
    let v1 = vec![1, 2, 3];
    let mut vi_iter = v1.iter();

    assert_eq!(vi_iter.next(), Some(&1));
    assert_eq!(vi_iter.next(), Some(&2));
    assert_eq!(vi_iter.next(), Some(&3));
    assert_eq!(vi_iter.next(), None);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
}
