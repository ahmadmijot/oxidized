fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1 //why x value can change?
    };

    println!("The value of \n x is: {} \n y is: {}", x, y);
}

fn another_function(i: i32, j: i32) {
    println!("The value of \n i is: {} \n j is: {}", i,j);
}