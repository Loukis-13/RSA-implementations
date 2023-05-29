use std::fs::File;
use std::io::prelude::*;
use rug::Integer;
use rug::integer::Order;

use super::primes;

pub fn keygen(p: &Option<Integer>, q: &Option<Integer>, bit_length: u32, name: String, print: bool) {
    let p = p.to_owned().unwrap_or(primes::gen_prime(bit_length));
    let q = q.to_owned().unwrap_or(primes::gen_prime(bit_length));
    let n = p.to_owned() * q.to_owned();
    let o = (p - 1u32) * (q - 1u32);
    let e = primes::gen_coprime(&o);
    let d = e.to_owned().invert(&o).unwrap();

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

pub fn encrypt(text: String, keys: (&Integer, &Integer)) {
    let x = Integer::from_digits(text.as_bytes(), Order::Msf);
    let (e, n) = keys;
    println!("{}", x.pow_mod(e, n).unwrap());
}

pub fn decrypt(text: String, keys: (&Integer, &Integer)) {
    let (d, n) = keys;
    let x = Integer::from(Integer::parse(text).unwrap()).pow_mod(d, n).unwrap();
    println!("{}", String::from_utf8_lossy(&x.to_digits::<u8>(Order::Msf)));
}
