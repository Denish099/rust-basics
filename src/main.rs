fn main() {
    let mut x: i8 = 2;  // make x mutable and use bigger type
    for i in 1..1000 {
        x = x + 100;
    }
    println!("Final x = {}", x);
}

// cargo init to intialize a rust project
// cargo run to run it 