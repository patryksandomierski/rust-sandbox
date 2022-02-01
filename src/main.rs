fn main() {
    let some_str = String::from("Hello, world!");
    let (str2, len) = tuple_to_the_rescue(some_str);
    println!("str={} has len={}", str2, len);
}

fn tuple_to_the_rescue(str: String) -> (String, usize) {
    let length = str.chars().count();
    (str, length)
}