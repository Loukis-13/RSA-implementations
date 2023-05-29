import gmpy2

RAND_STATE = gmpy2.random_state()


def gen_prime(bit_len=128):
    assert bit_len > 1
    return gmpy2.next_prime(gmpy2.mpz_urandomb(RAND_STATE, bit_len).bit_set(bit_len - 1))


def gen_coprime(o):
    while x := gmpy2.mpz_random(RAND_STATE, o):
        if gmpy2.gcd(x, o) == 1:
            return x


def keygen(p, q, bit_len=128):
    p = p or gen_prime(bit_len)
    q = q or gen_prime(bit_len)
    n = gmpy2.mul(p, q)
    o = gmpy2.mul(gmpy2.sub(p, 1), gmpy2.sub(q, 1))
    e = gen_coprime(o)
    d = gmpy2.invert(e, o)

    return ((e, n), (d, n))


def encrypt(text, e, n):
    x = gmpy2.mpz(text.encode().hex(), 16)
    return gmpy2.powmod(x, e, n)


def decrypt(text, d, n):
    x = gmpy2.powmod(gmpy2.mpz(text, base=16), d, n)
    return gmpy2.to_binary(x)[::-1].decode()
