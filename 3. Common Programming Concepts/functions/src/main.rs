fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is 5: {}", x);
    println!("The value of x 5+1: {}", plus_one(x));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is:{value}{unit}");
}

// return last expression
fn five () -> i32 {
    5
}

fn plus_one(x:i32)->i32{
    x+1
}
