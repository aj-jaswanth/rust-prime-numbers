pub struct PrimeGenerator {
    candidate: u64,
    primes: Vec<u64>,
}

impl PrimeGenerator {
    pub fn new() -> PrimeGenerator {
        PrimeGenerator {
            candidate: 2,
            primes: Vec::new(),
        }
    }

    fn is_prime(&self, n: u64) -> bool {
        for prime in &self.primes {
            if n % prime == 0 {
                return false;
            }
        }
        true
    }

    pub fn nth(&mut self, nth: usize) -> u64 {
        while self.primes.len() < nth {
            if self.is_prime(self.candidate) {
                self.primes.push(self.candidate);
            }
            self.candidate += 1;
        }
        self.primes[nth - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::prime::PrimeGenerator;
    use std::collections::HashMap;

    #[test]
    fn returns_the_nth_prime_number() {
        let mut prime_generator = PrimeGenerator::new();

        let setup = HashMap::from([
            (10, 29),
            (100, 541),
            (1000, 7919),
            (10000, 104729),
            (1, 2),
            (2, 3),
            (6, 13),
        ]);

        for (nth, prime) in setup {
            assert_eq!(prime_generator.nth(nth), prime);
        }
    }
}
