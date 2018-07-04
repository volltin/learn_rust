fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s: {}", s); // value used here after move

    let x = 5;
    makes_copy(x);
    println!("x: {}", x); // ok

    let s = gives_ownership();
    println!("s: {}", s);
    let s = takes_ownership_gives_back(s);
    println!("s: {}", s);

    let (s, sz) = two_rets(s);
    println!("s, sz: {} {}", s, sz);
}

fn takes_ownership(s: String) {
    println!("(takes_ownership) s: {}", s);
}

fn makes_copy(x: i32) {
    println!("(makes_copy) x: {}", x);
}

fn gives_ownership() -> String {
    String::from("~gives_ownership~")
}

fn takes_ownership_gives_back(s: String) -> String {
    println!("(takes_ownership_gives_back) s: {}", s);
    s
}

fn two_rets(s: String) -> (String, usize) {
    println!("(two_rets) s: {}", s);
    /*
    !!! directly return (s, s.len()) will failed with `s.len(): value used here after move`
    */
    let len = s.len();
    (s, len)
}