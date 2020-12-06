fn main() {
//mut
    let mut x = 5;
println!("the value of x is: {}", x);
x = 6;
println!("the value of x is: {}", x);

//shadowing
let y = 5;
let y = y + 1;
let y = y * 2;
println!("the value of y is: {}", y);

//change var type by shadowing
let spaces = "    ";
let spaces = spaces.len();
// error
// let mut spaces = "   ";
// spaces = spaces.len();

const MAX_POINTS: u32 = 100_000;
println!("{}",MAX_POINTS);
}
