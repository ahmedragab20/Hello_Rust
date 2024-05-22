fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    x.reverse();
    x.push(6);
    x.push(7);
    x.push(8);
    x.push(9);
    x.push(10);

    println!("The value of the last element is: {:?}", x);
}
