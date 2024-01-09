fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // error: cannot assign twice to immutable variable `x`
    x = 10;
    println!("The value of x is: {}", x);    
    
    let y = "   "; // immutable
    let y = y.len(); // shadowing
    println!("The value of y is: {}", y);

    let mut spaces = "   ";
    spaces = spaces.len(); // error: expected `&str`, found `usize`

}
