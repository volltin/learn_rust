const MAXV: u32 = 1000;
const SUM: u32 = 1000;

fn u32_sqrt(x: u32) -> Option<u32> {
    let t = (x as f64).sqrt() as u32;
    if t * t == x {
        return Some(t);
    } else if (t + 1) * (t + 1) == x {
        return Some(t + 1);
    } else if (t - 1) * (t - 1) == x{
        return Some(t - 1);
    } else {
        return None;
    }
}

pub fn find() -> Option<u32> {
    for a in 1..MAXV {
        for b in a..MAXV {
            let result = u32_sqrt(a * a + b * b);
            match result {
                None => continue,
                Some(c) => {
                    if a + b + c == SUM {
                        println!("a, b, c: {} {} {}", a, b, c);
                        return Some(a * b * c);
                    }
                }
            }
        }
    }
    return None
}
