use std::io;

fn main() {
    println!("input something below");


    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed");

    println!("You inputed: {}", &input_str)


}
