import math
import random

# fmt: off
FIRST_PRIMES = (
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
    113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
    241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
    383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521,
    523, 541,
)
# fmt: on


def gen_prime(bit_len=128):
    assert bit_len > 1

    x = random.randint(1 << (bit_len - 1), (1 << bit_len) - 1)
    x |= 1

    while True:
        if is_prime(x):
            return x
        x += 2


def gen_coprime(o):
    while True:
        x = random.randint(1, o - 1)
        if math.gcd(x, o) == 1:
            return x


def is_prime(n):
    if n < 2:
        return False

    for i in FIRST_PRIMES:
        if n == i:
            return True

        if n % i == 0:
            return False

    # algorítmo de Rabin Miller
    r = 0
    d = n - 1

    while d % 2 == 0:
        d >>= 1
        r += 1

    assert d * 2**r == n - 1

    for i in range(20):
        x = random.randrange(2, n)

        if pow(x, d, n) in (1, n-1):
            continue

        for i in range(r):
            if pow(x, d * 2**i, n) in (1, n-1):
                break
        else:
            return False

    return True
