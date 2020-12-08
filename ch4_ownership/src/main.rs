fn main() {
    string_type();
}

fn string_type() {
    let s = String::from("This is tring literal");
    let mut t = String::from("This can be mutated");
    t.push_str("like this");
    println!("{},{}",s, t);
}