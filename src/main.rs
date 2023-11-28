fn main() {
    let mut s = String::from("hello");

    {
        change(&mut s);
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
