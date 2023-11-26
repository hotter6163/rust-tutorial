fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);
}

/// この関数は引数として整数を受け取り、その値を表示します
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}
