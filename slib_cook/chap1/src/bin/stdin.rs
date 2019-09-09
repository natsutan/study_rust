use std::io;
use std::io::prelude;
use std::io::BufRead;

fn main()
{
    print_single_line("name ?");
    let for_name = read_line_iter();

    print_single_line("sur?:");
    let sur = read_line_buffer();

    print!("Hello {} {} ", for_name, sur);
}

fn print_single_line(text: &str)
{
    println!("{}", text);
    //io::stdout().flush().unwrap();

}

fn read_line_iter() -> String
{
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();
    input.expect("no line").expect("Fail").trim().to_string()

}

fn read_line_buffer() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    input.trim().to_string()fda
}