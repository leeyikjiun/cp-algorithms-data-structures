vector<int> sieveOfEratosthenes(int n) {
    vector<int> primes;
    if (n <= 1) return primes;

    vector<int> isPrime(n + 1, true);
    isPrime[0] = false;
    isPrime[1] = false;

    int i = 2;
    primes.push_back(i);
    for (int j = i * i; j <= n; j += i) {
        isPrime[j] = false;
    }
    for (i = 3; i * i <= n; i += 2) {
        if (isPrime[i]) {
            primes.push_back(i);
            for (int j = i * i; j <= n; j += i) {
                isPrime[j] = false;
            }
        }
    }
    for (; i <= n; i += 2) {
        if (isPrime[i]) primes.push_back(i);
    }
    return primes;
}
