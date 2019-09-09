use std::env;

fn main()
{
    println!("Listing all vars");
    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }

}