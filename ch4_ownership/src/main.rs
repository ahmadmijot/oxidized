fn main() {
   let s = String::from("hello"); // s comes into scope

   takes_ownership(s);

   let x = 5;
   makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}


// fn string_type() {
//     let s = String::from("This is string literal \n");
//     let mut t = String::from("This can be mutated \n");
//     t.push_str("like this");
//     let r = t;
//     println!("{},{}",r, s);
// }