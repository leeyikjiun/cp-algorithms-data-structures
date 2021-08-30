struct FenwickTree2D {
    bit: Vec<Vec<i32>>,
}

impl FenwickTree2D {
    fn with_capacity(m: usize, n: usize) -> Self {
        Self { bit: vec![vec![0; n]; m] }
    }
    
    fn sum(&self, mut x: usize, mut y: usize) -> i32 {
        let mut ret = 0;
        loop {
            loop {
                ret += self.bit[x][y];
                y &= y + 1;
                if y == 0 { break; }
                y -= 1;
            }
            x &= x + 1;
            if x == 0 { break; }
            x -= 1;
        }
        ret
    }
    
    fn add(&mut self, mut x: usize, mut y: usize, delta: i32) {
        while x < self.bit.len() {
            while y < self.bit[0].len() {
                self.bit[x][y] += delta;
                y |= y + 1;
            }
            x |= x + 1;
        }
    }
}
