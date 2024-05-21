pub fn vectors_sample() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);

    match v.get(0) {
        Some(value) => println!("v::: {}", value),
        None => println!("No value found"),
    }

    // println!("v: {}", v.get(0).unwrap());
}

pub fn vectors_sample2() -> Vec<i32> {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    return v;
}
