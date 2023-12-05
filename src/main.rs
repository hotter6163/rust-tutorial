extern crate rust_tutorial;

use std::env;
use std::process;

use rust_tutorial::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数の解析に失敗しました
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rust_tutorial::run(config) {
        // アプリケーションエラー: {}
        println!("Application error: {}", e);

        process::exit(1);
    }
}
