pub fn say_hi() {
    println!("Hello, world!");
}

pub mod math {
    pub fn add(a: u8, b: u8) -> u8 {
        a + b
    }

    pub fn dbl(a: u8, b: u8) -> u8 {
        self::add(a, b) * 2
    }

    pub fn sub(a: u8, b: u8) -> u8 {
        super::math::add(a, b)
    }
}
