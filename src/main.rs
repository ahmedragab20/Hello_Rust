mod chapter8;
fn main() {
    chapter8::vectors_sample();
    let mut v: Vec<i32> = chapter8::vectors_sample2();
    println!("v: {:?}", v);
    {
        let v_copy = &mut v;

        v_copy.push(6);
        println!("v_copy: {:?}", v_copy);
    }

    for i in &mut v {
        *i += 50;
        println!("i: {}", i);
    }

    println!("v_2: {:?}", v);
}
