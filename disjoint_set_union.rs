struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DisjointSetUnion {
    fn with_capacity(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find_set(&self, v: usize) -> usize {
        if v == self.parent[v] { v }
        else { self.find_set(self.parent[v]) }
    }

    fn union_sets(&mut self, mut a: usize, mut b: usize) {
        a = self.find_set(a);
        b = self.find_set(b);
        if a == b { return; }

        if self.size[a] < self.size[b] { std::mem::swap(&mut a, &mut b); }
        self.parent[b] = a;
        self.size[a] += self.size[b];
    }
}
