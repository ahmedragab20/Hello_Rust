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


    pub mod tests {

        pub fn test_add() {
            assert_eq!(super::add(1, 2), 3);
        }

        fn test_dbl() {
            assert_eq!(super::dbl(1, 2), 6);
        }

        fn test_sub() {
            assert_eq!(super::sub(1, 2), 3);
        }
    }
}
