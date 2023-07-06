void dijkstra(int s, vector<vector<pair<int, int>>>& adj, vector<int>& dist) {
    priority_queue<pair<int, int>> pq;
    pq.emplace(0, s);
    dist[s] = 0;
    while (!pq.empty()) {
        auto [d, u] = pq.top(); pq.pop();
        if (-d != dist[u]) continue;
        for (auto [v, w] : adj[u]) {
            if (w-d < dist[v]) {
                dist[v] = w-d;
                pq.emplace(d-w, v);
            }
        }
    }
}
