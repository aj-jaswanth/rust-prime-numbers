use crate::prime::PrimeGenerator;

mod prime;

fn main() {
    let mut prime_generator = PrimeGenerator::new();

    println!("{}", prime_generator.nth(100))
}
