# Section1

## Types of variables

---  

### Primitive types

[Rust doc primitive types](https://doc.rust-lang.org/std/index.html#primitives)

#### Integer types

--- 

##### Notes

- Each signed variant can store numbers from -(2n - ^1) to 2n -^1 -1.  
  - n is the number of bits that variant uses  
  - i8 can store numbers from -(2^7) to 2^7 -1  
- Unsigned cannot store negative values.  
  - u8 can store numbers from 0 to 2^8 -1 with equals 0 to 255.  
  - When you store a value beyond it's data type range then it is known as an overflow. ie... 256 becomes 0 and 257 becomes 1.  


---  

###### Signed

- i8  
- i16  
- i32
- i64  
- i128  
- isize


###### Unsigned

- u8  
- u16  
- u32  
- u64  
- u128  
- usize

---

#### Float types

- Rust has 2 primitive float types for floating point numbers.
  - f32
  - f64

---

#### Booleans

---

#### Charcter

---

### Mutability

- By default variables are immutable.
- When a variable is immutable and a value is bound to a name it cannot change value.

### Constants

- Constants are values that are bound to a name and are not allowed to change.
- Cannot use *mut* keyword to create a const
- Use ```const``` keyword to create a const
- Type of the value must be annotated.
- Constants can be declared in any scope, including the global scope.
- Constants may be set only to a constant expression
- Unmutable variables are not constants

### Strings

[Rustdoc on Strings](https://doc.rust-lang.org/std/string/index.html)

- Rust has only one string type in the core language, which is the string slice (&str)
- String type is provided by rust's standard library rather than the core language.
- String is growable, mutable, UTF-8 encoded.
- To create new string:
  - `String::new();`
  - `String::from("Hello");`

## Operators

- `+`
- `-`
- `/`
- `*`
- `%`
- `++ (not supported)`
- `-- (not supported)`
- `>`
- `<`
