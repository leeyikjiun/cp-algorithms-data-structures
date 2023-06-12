fn wheel_factorization(mut num: i32, primes: &Vec<i32>) -> Vec<i32> {
    let mut factors = Vec::new();
    for &prime in primes {
        if prime * prime > num { break }
        if num % prime == 0 {
            factors.push(prime);
            while num % prime == 0 {
                num /= prime;
            }
        }
    }
    if num > 1 { factors.push(num) }
    factors
}
