fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    const MAX_V: u32 = 100_000;
    println!("MAX_V: {}", MAX_V);

    let y = 233;
    println!("y: {}", y);
    let y = 'c'; // shadow y (y=233)
    println!("y: {}", y);
    let y = y; // shadow y (y=111)
    println!("y: {}", y);

}
