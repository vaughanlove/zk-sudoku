use std::num::Wrapping;

pub struct SimpleRng {
    state: Wrapping<u32>,
}

impl SimpleRng {
    pub fn new(seed: u32) -> Self {
        SimpleRng {
            state: Wrapping(seed),
        }
    }

    // xorshift32 for non-cryptographic deterministic randomness.
    fn next(&mut self) -> u32 {
        let mut x = self.state.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = Wrapping(x);
        x
    }

    fn gen_from_range(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next() % (max - min))
    }
}

pub fn generate_unique_array(rng: &mut SimpleRng) -> [u8; 9] {
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Fisher-Yates shuffle
    // in-place, O(n), each perm is uniformly distributed
    for i in (1..array.len()).rev() {
        let j = (rng.next() % (i as u32 + 1)) as usize;
        array.swap(i, j);
    }
    array
}

#[cfg(test)]
mod random_tests {
    use super::*;

    #[test]
    fn test_random_array() {
        let mut rng = SimpleRng::new(1);
        let random_array = generate_unique_array(&mut rng);
        assert_eq!(
            random_array,
            [3, 5, 8, 9, 4, 6, 7, 2, 1],
            "random generator is not deterministic"
        );
    }
}
