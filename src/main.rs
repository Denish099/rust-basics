
// fn main() {
    // let mut x: i8 = 2;  // make x mutable and use bigger type
    // for i in 1..1000 {
    //     x = x + 100;
    // }
    // println!("Final x = {}", x);
// let greeting: String = String::from("hello world");

//     println!("{}", greeting);

//     let ch = greeting.chars().nth(0);

//     match ch {
//         Some(c) => print!("{}", c),
//         None => print!("no character found at index"),
//     }
// }
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
    let s = String::from("hello");
    let s2 = String::from("world");

    let combined = format!("{} {}",s,s2);

    print!("{} {} {:p}",s2.len(),s2.capacity(),s2.as_ptr());
}