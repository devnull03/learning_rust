use std::io;

fn main() {
    println!("Rust Calculator");

    loop {
        println!("Choose operation: Add [a], Subtract [s], multiply [m], divide [d], exponent [e], Quit [q]");

        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("what shit did you enter here bro");

        // println!("{}", input_str.len());

        if input_str.len() != 2 {
            println!("Invalid operator");
            continue;
        }

        let op: String = input_str.clone();
        if op.trim().eq_ignore_ascii_case("q") {
            break;
        }
        println!("first num:");
        input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("what shit did you enter here bro");

        let x: f32 = match input_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid Float");
                continue;
            }
        };

        println!("second num:");
        input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("what shit did you enter here bro");

        let y: f32 = match input_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid Float");
                continue;
            }
        };

        if op.trim().eq_ignore_ascii_case("a") {
            println!("Result: {}", x + y)
        } else if op.trim().eq_ignore_ascii_case("s") {
            println!("Result: {}", x - y)
        } else if op.trim().eq_ignore_ascii_case("m") {
            println!("Result: {}", x * y)
        } else if op.trim().eq_ignore_ascii_case("d") {
            if y == 0.0 {
                println!("Cannot divide by zero!");
                continue;
            }
            println!("Result: {}", x / y)
        } else if op.trim().eq_ignore_ascii_case("e") {
            println!("Result: {}", x.powf(y))
        } else {
            println!("unavailable operator input: {}", &input_str)
        }
    }
}
