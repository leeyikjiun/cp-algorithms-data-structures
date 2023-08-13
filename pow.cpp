int pow(long long b, int e, long long m) {
    b %= m;
    long long c = 1;
    while (e) {
        if (e & 1) c = c * b % m;
        b = b * b % m;
        e >>= 1;
    }
    return c;
}
