// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("guess: {}", guess);

//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("x: {}, y: {}", x, y);

//     let sum = 5 + 10;
//     let difference: f64 = 95.5 - 4.3;
//     let quotient = 56.7 / 32.2;
//     let remainder = 43 % 5;
//     println!(
//         "sum: {}, difference: {}, quotient: {}, remainder: {}",
//         sum, difference, quotient, remainder
//     );

//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("x: {}, y: {}, z: {}", x, y, z);

//     const LENGTH: usize = 5;
//     let length: usize = 5;
//     let a = [3; LENGTH];
//     println!("a: {:?}", a);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number"); // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index,
        element
    );
}
