struct FenwickTree {
    bit: Vec<i32>,
}

impl FenwickTree {
    fn with_capacity(n: usize) -> Self {
        Self { bit: vec![0; n] }
    }

    fn sum(&self, mut r: usize) -> i32 {
        let mut ret = 0;
        loop {
            ret += self.bit[r];
            r &= r + 1;
            if r == 0 { break; }
            r -= 1;
        }
        ret
    }

    fn sum_range(&self, l: usize, r: usize) -> i32 {
        let mut ret = self.sum(r);
        if l > 0 { ret -= self.sum(l - 1); }
        ret
    }

    fn add(&mut self, mut idx: usize, delta: i32) {
        while idx < self.bit.len() {
            self.bit[idx] += delta;
            idx |= idx + 1;
        }
    }
}
