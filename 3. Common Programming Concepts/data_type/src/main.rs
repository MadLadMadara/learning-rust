fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // type annotation
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10; // i32
    let difference = 95.5 - 4.3; // f64
    let product = 4 * 30; // i32
    let quotient = 56.7 / 32.2; // f64
    let remainder = 43 % 5; // i32

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let five_hundred = x.0; // destructuring
    let six_point_four = x.1; // destructuring
    let one = x.2; // destructuring
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5]; // array
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

}
