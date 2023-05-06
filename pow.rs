fn pow(b: i32, mut e: u32, m: i32) -> i32 {
    let (mut b, m) = (b as i64, m as i64);
    b %= m;
    let mut c = 1_i64;
    while e > 0 {
        if e & 1 != 0 { c = c * b % m }
        b = b * b % m;
        e >>= 1;
    }
    c as i32
}
