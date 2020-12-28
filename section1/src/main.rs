#![allow(overflowing_literals)]

// io module to do input
use std::io;

// Constants in global
const GLOBAL_MAX: i32 = 500;

fn main() {
    let a: u128 = 1;
    println!("{}", a);

    // OVerflow
    let a: u8 = 257;
    println!("257 = {} when overflowing u8 datatype.", a);

    // Floats
    let a: f32 = 259.3;
    println!(" float types: {}", a);

    // bools
    let a: bool = true;
    println!("bool true is {}", a);

    // Charcter
    let a: char = 'a';
    println!("char a is {}", a);

    // Mutibility
    // we cannot do this:
    // a = 'b';
    // println!("{}", a);
    // we can do this:
    let mut b: char = 'a';
    println!("b is now {}", b);
    b = 'b';
    println!("b has been changed to {}", b);

    // constants
    const MAX: i32 = 10;
    println!("const value MAX is {}", MAX);
    println!("Global max is {}", GLOBAL_MAX);

    // Strings
    let a = "Hello";
    println!("String {}", a);

    let str: String = String::new();
    println!("Blank string {}", str);
    // cannot do this:
    // str = "blah".to_string();
    println!("changing string: {}", str);

    // vars are immutiable by default so we need to do this
    let mut mutiable_string: String = String::new();
    println!("Mutiable string start: {}", mutiable_string);
    mutiable_string = "blahblahblah".to_string();
    println!("Now changed string is: {}", mutiable_string);

    // You can also assign string like this:
    //let this_string: String = String::from("this string");
    let this_string = String::from("This string");
    println!("This string is: {}", this_string);

    // pushing to a string
    let mut push_string = String::from("blah");
    println!("String b4 push: {}", push_string);
    push_string.push_str(" de blah");
    println!("After the push: {}", push_string);

    /*
    Multiline comments
    blah
    */

    // Typecasting
    //use as keyword
    let a: i32 = 2;
    let b: i64 = 3;
    let a = b as i64;
    println!("a is {}", a);
    // can do this too
    let a: i32 = 22;
    let b: i64 = a.into();
    println!("b is: {}", b);

    // Taking input from a user
    let mut inputed_string=String::new();
    println!("Enter a String");
    io::stdin().read_line(&mut inputed_string).expect("Failed");
    println!("input is {}", inputed_string);

    // Take input for numbers or other types
    let mut input_num=String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input_num).expect("Failed");
    let input_num:i32=input_num.trim().parse().expect("Failed");
    println!("input num + 8 is: {}", input_num + 8);

}
