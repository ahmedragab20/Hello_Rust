use std::ops::Deref;

struct SBox<T>(T);

impl<T> Deref for SBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // self -> i32 -> 4 -> let num
        &self.0
    }
}

fn say_hi(name: &str) {
    println!("Hi, {name}");
}

fn main() {
    // let x = 4;
    // let y = Box::new(x);

    // let num = SBox(x);

    // assert_eq!(4, x);
    // assert_eq!(4, *num);

    let name = SBox(String::from("Ahmed"));
    say_hi(&name)
}
