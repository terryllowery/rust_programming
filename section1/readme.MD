# Section1

## Types of variables

---  

### Scaler

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