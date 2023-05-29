use num_bigint::{BigUint, RandBigInt, ToBigUint, ToBigInt};
use num_integer::Integer;
use num_traits::identities::{One, Zero};

static FIRST_PRIMES: [u64; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
    113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
    241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
    383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521,
    523, 541,
];

pub fn gen_prime(bits: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_biguint(bits);

    n.set_bit(bits-1, true);

    if n.is_even() {
        n += 1usize;
    }

    while !is_prime(&n) {
        n += 2usize;
    }

    n
}

pub fn gen_coprime(n: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_biguint_range(&One::one(), &(n.to_owned() - 1usize));

        if x.gcd(n).is_one() {
            return x;
        }
    }
}

pub fn modinv(a0: &BigUint, m0: &BigUint) -> BigUint {
    let m0 = m0.to_bigint().unwrap();

    if m0 == One::one() {
        return One::one();
    }

    let (mut a, mut m, mut x0, mut inv) = (a0.to_bigint().unwrap(), m0.to_bigint().unwrap(), 0.to_bigint().unwrap(), 1.to_bigint().unwrap());

    while a > One::one() {
        inv -= (&a / &m) * &x0;
        a = &a % &m;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
    }
    if inv < Zero::zero() {
        inv += m0
    }

    inv.to_biguint().unwrap()
}

pub fn is_prime(n: &BigUint) -> bool {
    for i in FIRST_PRIMES {
        if *n == i.to_biguint().unwrap() {
            return true;
        }

        if (n % i).is_zero() {
            return false;
        }
    }

    // algorítmo de Rabin Miller
    let mut r = 0;
    let mut d = n.to_owned() - 1usize;

    while !(d.to_owned() % 2usize).is_zero() {
        d >>= 1;
        r += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_biguint_range(&2.to_biguint().unwrap(), n);

        if x.modpow(&d, n).is_one() {
            continue;
        }

        'forelse: {
            for i in 0..r {
                if x.modpow(&(d.to_owned() * 2usize.pow(i)), n) == n - 1usize {
                    break 'forelse;
                }
            }
            return false;
        }
    }

    true
}
