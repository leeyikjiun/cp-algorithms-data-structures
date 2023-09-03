int l = ceil(log2(n));
int timer = 0;
vector<int> tin(n), tout(n);
vector<vector<int>> up(n, vector<int>(l));

function<void(int, int)> dfs = [&](int u, int p) {
    tin[u] = ++timer;
    up[u][0] = p;
    for (int i = 1; i <= l; ++i) {
        up[u][i] = up[up[u][i-1]][i-1];
    }
    for (int v : adj[u]) {
        if (v != p) dfs(v, u);
    }
    tout[u] = ++timer;
};

auto isAncestor = [&](int u, int v) {
    return tin[u] <= tin[v] && tout[u] >= tout[v];
};

auto lca = [&](int u, int v) {
    if (isAncestor(u, v)) return u;
    if (isAncestor(v, u)) return v;
    for (int i = l; i >= 0; --i) {
        if (!isAncestor(up[u][i], v)) u = up[u][i];
    }
    return up[u][0];
};
