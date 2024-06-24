fn get_full_name(first: &str, last: &str) -> String {
    let mut result: String = String::new();
    result.push_str(first);
    result.push_str(" ");
    result.push_str(last);

    return result;
}

fn main() {
    let full_name: String = get_full_name("ahmed", "ragab");

    println!("full name: {full_name}")
}
