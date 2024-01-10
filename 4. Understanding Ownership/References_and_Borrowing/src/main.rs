fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    } // s1 goes out of scope here, but because it does not have ownership of what
      // it refers to, nothing happens.
    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    }// s is a reference to a String

    // cant modify borrowed value unless it is mutable
    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s);
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    // can not create two mutable referances to the same variable
    { // will cause error
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s; // error
        println!("{}", r1);
    }
  
}


