fn main() {
    let x = 100;
    func(x, five());
}

fn func(x: i32, y: i32) {
    println!("func x y: {} {}", x, y);
}

fn five() -> i32 {
    5
}