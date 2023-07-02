fn sieve_of_eratosthenes(n: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    let n = n as usize;
    if n <= 1 { return primes }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    primes.push(i as i32);
    for j in (i * i..=n).step_by(i) {
        is_prime[j] = false;
    }
    i = 3;
    while (i * i <= n) {
        if is_prime[i] {
            primes.push(i as i32);
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
        i += 2;
    }
    for i in (i..=n).step_by(2) {
        if is_prime[i] { primes.push(i as i32) }
    }
    primes
}
