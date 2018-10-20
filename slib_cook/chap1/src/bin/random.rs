extern crate rand;

fn main()
{
    let random_num1 = rand::random::<i32>();
    let random_num2: i32 = rand::random();
    println!("rand {} {}", random_num1, random_num2);

    let random_char: char = rand::random();
    println!("rand {}", random_char);

    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        if rng.gen() {
            println!("success!");
        }
    }

    let random_num3 = rng.gen_range(0, 10);
    let random_num4 = rng.gen_range(0.0, 1.0);
    println!("random {} {} ", random_num3, random_num4);
}