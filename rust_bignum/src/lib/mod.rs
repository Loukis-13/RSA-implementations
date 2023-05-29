mod rsa;
mod primes;

pub use rsa::keygen;
pub use rsa::encrypt;
pub use rsa::decrypt;
pub use primes::is_prime;

