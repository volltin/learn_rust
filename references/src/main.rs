fn main() {
    // ref
    let s = String::from("hello");

    let l = calc_len(&s);

    println!("s len: {} {}", s, l);

    // mut ref
    let mut s = String::from("233");
    change(&mut s);
    println!("s: {}", s);

    // multiple ref
    let s = String::from("hello! ");

    let ref1 = &s;
    let ref2 = &s;
    println!("s ref1 ref2: {} {} {}", s, ref1, ref2);

    // multiple mut ref
    let mut s = String::from("hello! ");

    let ref1 = &mut s;
    //let ref2 = &mut s; // second mutable borrow occurs here
    //println!("s ref1 ref2: {} {} {}", s, ref1, ref2);
}

fn calc_len(s: &String) -> usize {
    //s.push_str(", world"); //cannot borrow as mutable
    println!("(calc_len) s: {}", s);
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
    println!("(change) s: {}", s);
}