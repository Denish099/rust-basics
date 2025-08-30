
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

fn main(){
    let x = 1;  //created on stack , owner is main function
    let y = 2;  //created on stack owner is main function
    println!("{}",sum(x,y));
    println!("hello world");
}

fn sum(a:i32,b:i32)->i32{
    let c = a+b;  //created a,b,c on stack owner is sum function
    return c;
}