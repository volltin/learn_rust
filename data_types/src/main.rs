/*
scalar types:
- integer: i32 u32 i64 u64 isize usize
- float: f32 f64
- bool: {true, false}
- char: unicode 'x'

compound types:
- tuple (different types)
- array (same type)

*/


fn main() {
    println!("Hello, world!");

    let tup: (u32, f32, i32) = (123, 456.1, 789);
    let (x, y, z) = tup;
    println!("x, y, z: {} {} {}", x, y, z);

    println!("tup.0 tup.1 tup.2: {} {} {}", tup.0, tup.1, tup.2);

    let a = [5, 4, 3, 2, 1];
    println!("a[3]: {}", a[3]);
    //println!("a[3]: {}", a[6]); // panicked

    let index = 10;
    println!("index: {}", index);
    //println!("a[index]: {}", a[index]); // panicked
}
