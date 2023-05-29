mod primes;
mod rsa;

pub use primes::gen_prime;
pub use primes::is_prime;
pub use rsa::decrypt;
pub use rsa::encrypt;
pub use rsa::keygen;
