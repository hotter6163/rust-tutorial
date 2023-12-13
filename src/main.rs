struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名構造体
    Write(String),
    ChangeColor(i32, i32, i32), // タプル構造体
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        // それ以外
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // ASCII文字前半
        'a'..='j' => println!("early ASCII letter"),
        // ASCII文字後半
        'k'..='z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("a = {}, b = {}", x, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("{}", sum_of_squares);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

// https://doc.rust-jp.rs/book-ja/ch18-03-pattern-syntax.html#%E3%83%8D%E3%82%B9%E3%83%88%E3%81%95%E3%82%8C%E3%81%9F_%E3%81%A7%E5%80%A4%E3%81%AE%E4%B8%80%E9%83%A8%E3%82%92%E7%84%A1%E8%A6%96%E3%81%99%E3%82%8B

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
