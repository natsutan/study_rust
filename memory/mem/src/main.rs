use std::mem;

type Array4 = [i32; 4];
type OArray4 = [Option<i32>; 4];

fn main() {

    println!("size of i32 = {}", mem::size_of::<i32>());
    println!("size of i32 = {}", mem::size_of::<Option<i32>>());
    println!("size of [i64: 4] = {}", mem::size_of::<Array4>());
    println!("size of [Option<i64>: 4] = {}", mem::size_of::<OArray4>());

    let arr1 :[i32; 4] = [1, 2, 3, 4];
    let arr2 :[Option<i32>; 4] = [Some(1), Some(2), None, Some(3)];

    println!("arr1 = {:?}", arr1);
    let view = &arr1 as *const _ as *const u8;
    for i in 0 .. mem::size_of::<Array4>() as isize {
        print!("{:02x} ", unsafe {*view.offset(i)});
    }
    println!("");


    println!("arr2 = {:?}", arr2);
    let view = &arr2 as *const _ as *const u8;
    for i in 0 .. mem::size_of::<OArray4>() as isize {
        print!("{:02x} ", unsafe {*view.offset(i)});
    }
    println!("");

}
