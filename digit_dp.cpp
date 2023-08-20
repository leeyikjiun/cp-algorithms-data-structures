int memo[10][2];

vector<int> digits(int num) {
    vector<int> digits;
    while (num) {
        digits.push_back(num%10);
        num/=10;
    }
    reverse(digits.begin(), digits.end());
    return digits;
}

int dp(const vector<int>& digits, int i, int tight) {
    if (i == digits.size()) return 0;
    if (memo[i][tight] != -1) return memo[i][tight];
    int answer = 0;
    for (int j = tight ? digits[i] : 9; j >= 0; --j) {
        int newTight = tight && j == digits[i];
        answer += dp(digits, i+1, newTight);
    }
    return memo[i][tight] = answer;
}

int f(int low, int high) {
    memset(memo, -1, sizeof(memo));
    int a = dp(digits(high), 0, 1);
    memset(memo, -1, sizeof(memo));
    int b = dp(digits(low-1), 0, 1);
    return a - b;
}
