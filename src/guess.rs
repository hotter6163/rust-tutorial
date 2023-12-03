use core::result::Result::Err;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, &'static str> {
        if value < 1 || value > 100 {
            return Err("値は1から100の間でなければなりません");
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
