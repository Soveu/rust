// check-fail

fn main() {
    let x = y = 0;

    let mut a = 0i32;
    let b = a = 42i32;
    let c = a += 1;

    let d = c = b = a += 0;
    a = b = c;
}
