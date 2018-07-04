fn main() {
    let x = 1;

    let y = {x};
    println!("y: {}", y);

    let y = {x + 1};
    println!("y: {}", y);

    let y = {let x = x + 1; x + 1};
    println!("y: {}", y);

    let y = {x};
    println!("y: {}", y);
}
