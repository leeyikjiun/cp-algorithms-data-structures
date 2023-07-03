struct FenwickTree {
    vector<int> bit;

    FenwickTree(int n) {
        bit.assign(n, 0);
    }

    int sum(int r) {
        int ret = 0;
        for (; r >= 0; r = (r & (r + 1)) - 1) {
            ret += bit[r];
        }
        return ret;
    }

    int sum(int l, int r) {
        int ret = sum(r);
        if (l >= 1) ret -= sum(l - 1);
        return ret;
    }

    void add(int idx, int delta) {
        for (; idx < bit.size(); idx |= (idx + 1)) {
            bit[idx] += delta;
        }
    }
};
