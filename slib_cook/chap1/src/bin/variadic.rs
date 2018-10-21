
macro_rules! multiply {
    ( $last:expr ) => { $last };

    ( $head:expr, $($tail:expr), + ) => {
        $head * multiply!($($tail), +)
    };

}


fn main()
{
    let val1 = multiply!(2, 4, 8);
    println!("2 * 4 * 8 = {}", val1);

    let val2 = multiply!(2, 4, 8, 16);
    println!("2 * 4 * 8 * 16 = {}", val2);

}
