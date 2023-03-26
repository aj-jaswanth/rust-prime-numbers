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

    fn is_prime(n: u64) -> bool {
        let mut divisors = 0;
        for divisor in 1..=n {
            if n % divisor == 0 {
                divisors += 1;
            }
        }
        divisors == 2
    }

    pub fn nth(&mut self, nth: u64) -> u64 {
        while self.primes.len() < nth as usize {
            if PrimeGenerator::is_prime(self.candidate) {
                self.primes.push(self.candidate);
            }
            self.candidate += 1;
        }
        self.primes[(nth - 1) as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::prime::PrimeGenerator;

    #[test]
    fn verifies_a_number_is_prime_or_not() {
        assert_eq!(PrimeGenerator::is_prime(1), false);
        assert_eq!(PrimeGenerator::is_prime(2), true);
        assert_eq!(PrimeGenerator::is_prime(3), true);
        assert_eq!(PrimeGenerator::is_prime(4), false);
        assert_eq!(PrimeGenerator::is_prime(5), true);
    }

    #[test]
    fn returns_the_nth_prime_number() {
        let mut prime_generator = PrimeGenerator::new();

        assert_eq!(prime_generator.nth(6), 13);
        assert_eq!(prime_generator.nth(10), 29);
        assert_eq!(prime_generator.nth(100), 541);
        assert_eq!(prime_generator.nth(1), 2);
    }
}
