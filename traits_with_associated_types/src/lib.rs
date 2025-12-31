use std::ops::Add;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, othrer: Meters) -> Millimeters {
        Millimeters(self.0 + (othrer.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your caption speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}
