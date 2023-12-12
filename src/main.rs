#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match self.list.len() {
            0 => {
                self.average = 0.0;
            }
            _ => {
                self.update_average();
            }
        }
        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    println!("{:?}", c);

    c.add(5);
    println!("{:?}", c);

    c.add(10);
    println!("{:?}", c);

    c.remove();
    println!("{:?}", c);
}
