#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Copy, U> Point<T, U> {
    fn mixup<V, W: Copy>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let both_integer = Point {
        x: String::from("hello world"),
        y: 10,
    };
    let both_float = Point { x: 1.0, y: 4.0 };
    println!("both_integer.x = {}", both_integer.x());
    println!("both_float.x = {}", both_float.x());
    println!(
        "both_float.distance_from_origin = {}",
        both_float.distance_from_origin()
    );
    println!("{:?}", both_float.mixup(&both_integer));
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
