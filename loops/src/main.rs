fn main() {
    let mut x = 1;

    while x != 5 { x = x + 1 };

    println!("x: {}", x);

    let a = [0, 1, 2, 3, 4, 5];

    for item in a.iter() {
        println!("item: {}", item);
    }

    for item in (0..6).rev() {
        println!("item: {}", item);
    }
}
