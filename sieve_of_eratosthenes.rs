fn sieve_of_eratosthenes(n: i32) -> Vec<i32> {
    let mut primes = vec![2];
    let n = n as usize;
    let mut is_prime = vec![true; n+1];
    for i in (3..=n).step_by(2) {
        if is_prime[i] {
            primes.push(i as i32);
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    primes
}
