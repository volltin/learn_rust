fn main() {
    let s = "hello";
    println!("s: {}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("s: {}", s);

    // move
    let s1 = String::from("s1s1s11");
    let s2 = s1;
    // println!("s1: {}", s1); // value has moved
    println!("s2: {}", s2);

    // clone
    let s1 = String::from("s1s1s11");
    let s2 = s1.clone();

    println!("s1: {}", s1);
    println!("s2: {}", s2);

    // still clone
    let x1 = 5;
    let x2 = x1;
    println!("x1: {}", x1);
    println!("x2: {}", x2);
}
