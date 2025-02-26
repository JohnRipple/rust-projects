use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut text: String = "".to_string();
    for i in 1..args.len() {
        if args[i].starts_with("-") || args[i].starts_with("--") {
            match args[i].as_str() {
                "-h" | "--help" => { 
                    println!("This program echos text");
                    process::exit(0)
                },
                _ => println!("Unknown option"),
            }
        } else {
            text = args[i..].join(" ");
            break;
        }
    }
    println!("{}", text);
}
