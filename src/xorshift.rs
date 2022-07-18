pub struct Rng(u64);

impl Rng {
    /// Creates a new seeded PRNG
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Returns the next pseudo-randomly generated number
    pub fn rand(&mut self) -> u64 {
        let ret = self.0;
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 43;
        ret
    }

    /// Shuffle a vector in place
    pub fn shuffle(&mut self, vector: &mut Vec<usize>) {
        let n_shuffles = vector.len();
        for i in 0..n_shuffles {
            let random_idx: usize = self.rand() as usize % n_shuffles;
            let temp = vector[i];
            vector[i] = vector[random_idx];
            vector[random_idx] = temp;
        }
    }
}
