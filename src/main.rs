
// cargo init to intialize a rust project
// cargo run to run it 


// string in rust does not have fixed type
// string can change spadce in runtime 



// &str (string slice)

// A reference to a string slice (borrowed string).

// Immutable by default.

// Usually points to a fixed string in memory (e.g., string literals) or a part of another string.

// Stored on the stack as a pointer + length, but the actual string data can live elsewhere (like in the program’s binary for literals).

// Lightweight and faster when you don’t need ownership.

// String

// An owned, growable string type.

// Heap-allocated (can store dynamic content at runtime).

// Mutable (you can push, insert, or change contents).

// Stores data as a UTF-8 encoded sequence.

// When you need to own the string and modify it, use String.


// irresplective of string number all data types in rust are immutable
// to make it mutable use mut after let 

// fn main(){
//     let x = 1;  //created on stack , owner is main function
//     let y = 2;  //created on stack owner is main function
//     println!("{}",sum(x,y));
//     println!("hello world");
// }

// fn sum(a:i32,b:i32)->i32{
//     let c = a+b;  //created a,b,c on stack owner is sum function
//     return c;
// }


// fn main(){
//     let s1 = String::from("hi there");
//     print!("{}",s1);
//     let s2 = s1;
//     print!("{}",s2);
// }

//if the owner goes out of scope and the coressponding vaiabkealso goes out of scope so this is why rust is so safe

// fn main(){
//     let s1= String::from("hello");
//     takeownership(s1);

//     print!("{}",s1.clone());
// }

// fn takeownership(some_string:String){
//     print!("{}",some_string);
// }

// fn main(){

//     let s1:String = String::from("hello");
//     let s2 = &s1;

//     print!("{}",s1);
//     print!("{}",s2);


// }

// fn main(){
//     let s1:String = String ::from("hello");

//     change_ownership(&s1);
//     print!("{}",s1);

// }

// fn change_ownership(some_string:&String){
//     println!("{}",some_string);  // this string is borrowed no ownership is changed 
// }


// fn main() {
//     let mut s1: String = String::from("hello");
//     update_word(&mut s1);
//     println!("{}", s1);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" world");
// }


//if something is borrowed mutably it can not be borrowed again



//lifetimes
//string slices
 

// struct reactangle{
//     width:u32,
//     height:u32
// }

// impl reactangle {
//     fn area(&self)-> u32{
//         self.width * self.height
//     }
    
// }

// fn main(){
//     let react = reactangle{
//         width:10,
//         height:19
//     };

//     print!("{}",react.area());

// }