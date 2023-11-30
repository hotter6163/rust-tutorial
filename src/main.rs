#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!(
        "{} {} {} {}",
        value_in_cents(Coin::Penny),
        value_in_cents(Coin::Nickel),
        value_in_cents(Coin::Dime),
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    let option = plus_one(Some(5));
    println!("{:?}", option);
}
