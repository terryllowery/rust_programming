fn main() {
    let a = 10;

    if a % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // else if flows
    if a == 0 {
        println!("A is 0");
    } else if a < 5 {
        println!("a is less then 5");
   } else if a % 2 == 0 {
       println!("a is even");
   } else {
       println!("a is odd");
   }

   let var_name = if a % 2 == 0 {
       "Even" // no semicolon
   } else {
       "Odd" // no semicolon
   }; // must be closed by semicolon since it is a statement

   println!("in let var_name a is: {}", var_name);
}
