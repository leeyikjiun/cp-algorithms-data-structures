vector<int> wheelFactorization(int num, vector<int>& primes) {
    vector<int> factors;
    for (int prime : primes) {
        if (prime * prime > num) break;
        if (num % prime == 0) {
            factors.push_back(prime);
            while (num % prime == 0) num /= prime;
        }
    }
    if (num > 1) factors.push_back(num);
    return factors;
}
