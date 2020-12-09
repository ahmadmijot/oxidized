//refactor using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        // :? is debug, need annotation
        // :#? is prettier than :?
        "The rect is {:#?} and the area of the rect is {} square pixels",
        rect1,
        area(&rect1)
    );
}

fn area(rectangle: & Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


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