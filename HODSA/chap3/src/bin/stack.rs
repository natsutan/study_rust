use std::mem;

struct Mystruct {
    a:u8,
    b:u8,
    c:u8
}

fn main() {
    assert_eq!(mem::size_of::<Mystruct>(), 3 * mem::size_of::<u8>());
    assert_eq!(mem::size_of::<[Mystruct; 2]>(), 3 * mem::size_of::<u8>() * 2);
}