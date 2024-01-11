#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    // Refactor with Tuples
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );
    // Refactor with Structs
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("rect1 is {:#?}", rect2);
    dbg!(&rect2);

}

fn area(w:i32, h:i32) -> i32 {
    return w * h;
}

fn area2(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1;
}

fn area3(rectangle: &Rectangle) -> i32 {
    return rectangle.width * rectangle.height;
}
