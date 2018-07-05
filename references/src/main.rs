/*

At any time:
- only one mut ref.
- or, many imut refs.

ref must be valid (not dangle).
*/

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
    println!("ref1: {}", ref1);

    // data race
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", world1");
        println!("(scope) r1: {}", r1);
    }

    let r2 = &mut s;
    r2.push_str(", world2");
    println!("r2: {}", r2);

    // imut + mut ref
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; //mutable borrow occurs here
    println!("s, r1, r2: {} {} {}", s, r1, r2);
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

/* expected lifetime parameter
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn not_dangle() -> String {
    let s = String::from("hello");
    s
}

