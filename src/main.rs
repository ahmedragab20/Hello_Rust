mod helpers;
use helpers::{say_hi, math::add};
fn main() {
    // sick..
    say_hi();
    let three: u8 = add(1, 2);

    println!("1 + 2 = {}", three);
}