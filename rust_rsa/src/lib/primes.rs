use rug::{rand::RandState, Integer, Assign};
use std::time::{UNIX_EPOCH, SystemTime};

static FIRST_PRIMES: [u16; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
    113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
    241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
    383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521,
    523, 541,
];

pub fn gen_prime(bits: u32) -> Integer {
    let mut rand = RandState::new();
    rand.seed(&Integer::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));

    let mut n = Integer::from(Integer::random_bits(bits, &mut rand));

    n |= 1;

    while !is_prime(&n) {
        n += 2usize;
    }

    n
}

pub fn gen_coprime(n: &Integer) -> Integer {
    let mut rand = RandState::new();
    rand.seed(&Integer::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));

    loop {
        let x = Integer::random_below(n.to_owned(), &mut rand);

        if x.to_owned().gcd(n) == 1 {
            return x;
        }
    }
}

pub fn is_prime(n: &Integer) -> bool {
    for i in FIRST_PRIMES {
        if *n == i {
            return true;
        }

        if n.to_owned() % i == 0 {
            return false;
        }
    }

    // algorítmo de Rabin Miller
    let mut r = 0;
    let mut d = n.to_owned() - 1u8;

    while d.is_odd() {
        d >>= 1;
        r += 1;
    }

    let mut rand = RandState::new();
    rand.seed(&Integer::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));

    for _ in 0..20 {
        let mut x = n.to_owned().random_below(&mut rand);

        if x < 2 {
            x.assign(2);
        }

        if x.to_owned().pow_mod(&d.to_owned(), n).unwrap() == 1 {
            continue;
        }

        'forelse: {
            for i in 0..r {
                if x.to_owned().pow_mod(&(d.to_owned() * 2usize.pow(i)), n).unwrap() == n.to_owned() - 1u8 {
                    break 'forelse;
                }
            }
            return false;
        }
    }

    true
}
