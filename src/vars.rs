// Variables hold primitive data or refrences to data structures
// Variables are immutable by default
// Rust is a block scoped language


pub fn run() {
 let name="Brime";
let mut age = 19;
println!("my name is {} and i am {}", name, age);
age=49;
 println!("my name is {} and i am {}", name, age);
 
 //define constants, Variable is usually in CAPS and type must be defined
 const ID: i32=001;
 
 println!("ID: {}", ID);

 //Assign multiple variables to a variable

 let (my_name, my_age) = ("Brime", 37);

 println!("my name is {name} and i am {age}", name= my_name, age= my_age);

}