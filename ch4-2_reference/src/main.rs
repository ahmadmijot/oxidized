




// fn main() {
//     //references
//     let s1 = String::from("just ref, can't change");
//     let len  =calculate_length(&s1);
//     println!("the length of {} is {}", s1, len);

//     let mut s = String::from("mutable");
//     change(&mut s);
//     println!("{}", s);

//     let mut t = String::from("can compile using curly bracket");
//     {
//         let t1 = &mut t;
//     } // t1 goes out of scope here

//     let t2 = &mut t;
// }


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(" and changable");
// }

// // this program will have error
// // let mut s = String::from("can only mut one ref");
// // let r1 = &mut s;
// // let r2 = &mut s;