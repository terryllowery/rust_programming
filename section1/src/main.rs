#![allow(overflowing_literals)]
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
}
