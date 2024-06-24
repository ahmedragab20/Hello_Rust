use std::fmt::Display;

fn longest<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
trait MyTrait {
    fn love(&self);
}
struct MyStruct;
impl MyTrait for MyStruct {
    fn love(&self) {
        println!("i love you 🥹")
    }
}
fn main() {
    let string1 = "Hello";
    let string2 = "world!";
    let result = longest(string1, string2, 42); // 42 is of type i32, which implements Display
    let structy = MyStruct {};
    structy.love();
    println!("The longer string is: {}", result);
}
