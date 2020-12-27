#![allow(overflowing_literals)]

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
 let this_string=String::from("This string");
 println!("This string is: {}", this_string);

 // pushing to a string
 let mut push_string=String::from("blah");
 println!("String b4 push: {}",push_string );
 push_string.push_str(" de blah");
 println!("After the push: {}", push_string);



}
