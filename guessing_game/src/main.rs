use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // create mut var that bound to the new, empty instnace of String

        io::stdin()
            .read_line(&mut guess) // & is reference
            .expect("Failed to read line"); //expect err or ok from std::result::Result
    
//        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
        // parse: return Result
        //expect: crashing on an error
        // match: handling the error

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}