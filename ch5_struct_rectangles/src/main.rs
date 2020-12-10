// refactor using methods
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

//Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.width > other.width && self.height > other.height
        self.area() > other.area() //my code
    }

    //associated func, called by sq1 = Rectangle::square(23);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//refactor using structs
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         // :? is debug, need annotation
//         // :#? is prettier than :?
//         "The rect is {:#?} and the area of the rect is {} square pixels",
//         rect1,
//         area(&rect1)
//     );
// }

// fn area(rectangle: & Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


//refactored with tuples
// fn main() {
//     let rect1 = (30,50);

//     println!(
//         "The area of the rec is {} square pixels.",
//         area(rect1)
//     )
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// ori fn
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rec is {} square pixels.",
//         area(width1,height1)
//     );
// }



// ori fn
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }