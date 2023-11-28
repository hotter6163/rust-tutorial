fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("The first word is: {}", word);
    let word = second_word(&s); // word will get the value 5
    println!("The second word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // iter() returns each element in a collection
        if item == b' ' {
            // b' ' is a byte literal
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = 0;
    let mut second_space = 0;

    for (i, &item) in bytes.iter().enumerate() {
        // iter() returns each element in a collection
        if item == b' ' {
            // b' ' is a byte literal
            if first_space == 0 {
                first_space = i;
            } else {
                second_space = i;
            }
        }
    }
    if (second_space == 0) {
        return &s[first_space + 1..];
    }

    &s[first_space + 1..second_space]
}
