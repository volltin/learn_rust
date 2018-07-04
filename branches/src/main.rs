fn main() {
    let number = 5;

    if number < 3 {
        println!("<3");
    } else {
        println!(">=3");
    }
    //if number { println!("/=0"); } else { println!("=0"); }
    if number != 0 { println!("/=0"); } else { println!("=0"); }

    let x = 18;
    if x % 4 == 0 {
        println!("x % 4: 0");
    } else if x % 4 == 1 {
        println!("x % 4: 1");
    } else {
        println!("x % 4: 2,3");
    }

    let cond = false;

    let res = if cond { 233 } else { 455 };
    println!("res: {}", res);

    //let res = if cond { 233 } else { 455.0 }; // E0308
    //println!("res: {}", res);
}
