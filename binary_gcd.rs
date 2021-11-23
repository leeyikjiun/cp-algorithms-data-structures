// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation
pub fn binary_gcd(mut u: i32, mut v: i32) -> i32 {
    if u == 0 { return v; }
    else if v == 0 { return u; }

    let i = u.trailing_zeros(); u >>= i;
    let j = v.trailing_zeros(); v >>= j;
    let k = i.min(j);

    loop {
        if u > v { std::mem::swap(&mut u, &mut v); }
        v -= u;

        if v == 0 { return u << k; }

        v >>= v.trailing_zeros();
    }
}
