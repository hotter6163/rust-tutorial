// extern crate rust_tutorial;

// use std::env;
// use std::process;
use std::thread;
use std::time::Duration;

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

// https://doc.rust-jp.rs/book-ja/ch13-01-closures.html#%E9%96%A2%E6%95%B0%E3%81%A7%E3%83%AA%E3%83%95%E3%82%A1%E3%82%AF%E3%82%BF%E3%83%AA%E3%83%B3%E3%82%B0
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;

    let equal_rt_x = |z| z == x;

    let y = 4;
    assert!(equal_rt_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_retult = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_retult.value(intensity));
        println!("Next, do {} situps!", expensive_retult.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_retult.value(intensity)
            );
        }
    }
}
