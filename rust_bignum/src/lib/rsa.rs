use std::fs::File;
use std::io::prelude::*;

use num_bigint::BigUint;

use super::primes;

pub fn keygen(p: &Option<BigUint>, q: &Option<BigUint>, bit_length: u64, name: String, print: bool) {
    let p = p.to_owned().unwrap_or(primes::gen_prime(bit_length));
    let q = q.to_owned().unwrap_or(primes::gen_prime(bit_length));
    let n = p.to_owned() * q.to_owned();
    let o = (p - 1u32) * (q - 1u32);
    let e = primes::gen_coprime(&o);
    let d = primes::modinv(&e, &o);

    if print {
    println!("Public  key: ({e}, {n})");
    println!("Private key: ({d}, {n})");
    }

    File::create(format!("{name}.pub"))
        .unwrap()
        .write_all(format!("{e} {n}").as_bytes())
        .unwrap();
    File::create(name)
        .unwrap()
        .write_all(format!("{d} {n}").as_bytes())
        .unwrap();
}

pub fn encrypt(text: String, keys: (&BigUint, &BigUint)) {
    let x = BigUint::from_bytes_be(text.as_bytes());
    let (e, n) = keys;
    println!("{}", x.modpow(e, n));
}

pub fn decrypt(text: String, keys: (&BigUint, &BigUint)) {
    let (d, n) = keys;
    let x = BigUint::parse_bytes(text.as_bytes(), 10).unwrap().modpow(d, n);
    println!("{}", String::from_utf8_lossy(&x.to_bytes_be()));
}
