fn main() {
    let some_str = String::from("Hello, world!");
    let len = but_better_using_refs(&some_str);
    println!("str={} has len={}", some_str, len);
}

fn but_better_using_refs(str: &String) -> usize {
    let length = str.chars().count();
    length
}