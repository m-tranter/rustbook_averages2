use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments.");
        process::exit(1);
    } else {
        let mut num: Vec<i32> = Vec::new();
        for a in &args[1..] {
            match a.parse::<i32>() {
                Ok(n) => num.push(n),
                Err(e) => {
                    println!("I can only accept integers as arguments.");
                    process::exit(1);
                }
            }
        }
        println!("{:?}", num);
    }
}
