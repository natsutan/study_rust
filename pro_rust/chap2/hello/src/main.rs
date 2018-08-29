use std::io::Write;
use std::str::FromStr;



#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);    
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && m != 0);
    while m != 0{
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }    
    n
}


fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument."));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "usage: gcd number").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    
    println!("Hello, world!");
    println!("GCD {:?} is {}.", numbers, d);
}

