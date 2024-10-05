fn main() {
    let x: &i32 = &get_number();
    println!("x: {}", x);
}

fn get_number() -> i32 {
    let x: i32 = 10;
    x
}
