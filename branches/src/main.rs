fn main() {
    // ifelse();
    // conditional();
    // returnloop();
    // whileloop();
    //for_loop_using_while();
    //for_loop();
    countdown_for();
}

fn ifelse() {
    let number = 6;

    if number % 4 == 0 {
        println!("this number is divisible by 4");
    } else if number % 3 == 0 {
        println!("this number is divisible by 3");
    } else if number % 2 == 0 {
        println!("this number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }
}

fn conditional() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is: {}", number);
}

fn returnloop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // need to put break counter
            // if break only will get error
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn whileloop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop_using_while() {
    // slower and not concise, more error prone
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    // faster and more concise
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn countdown_for() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
