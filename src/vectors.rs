// Arrays - Resizable arrays.

use std::mem;

pub fn run() {
 let mut numbers: Vec<i32>= vec![1,2,3,4,5];

 // Re-assign value
 numbers[2] =20;

 // Add to Vector
numbers.push(6);
numbers.push(7);

// Pop offlast value
numbers.pop();

 println!("{:?}", numbers);

 // Get single value
 println!("Single Value: {:?}", numbers[0]);

 // Get Vector length
 println!("Vector length: {}", numbers.len());

 // Vector are stack allocated
 println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

 // Get Slice
 let slice: &[i32] = &numbers[0..2];
 
 println!("Slice: {:?} ", slice);

 // loop through vector values
 for x in numbers.iter() {
  
  println!("Number: {}", x)
 }
 //loop & mutate value
 for x in numbers.iter_mut() {
  *x *= 2;
 }
 println!("Number Vec: {:?}", numbers)
}   