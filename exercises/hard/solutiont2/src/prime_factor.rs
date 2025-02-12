use rand::Rng;
use std::cmp;

pub fn find_max_prime_factor(number: u128) -> u128 {
    if number == 1 {
        return number;
    }
    let factors = factor(number);
    factors.into_iter().max().unwrap_or(number)
}

fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in &small_primes {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for &a in &small_primes {
        if a >= n {
            continue;
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut is_composite = true;
        for _ in 0..s - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                is_composite = false;
                break;
            }
        }
        if is_composite {
            return false;
        }
    }
    true
}

fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn pollards_rho(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }
    let mut rng = rand::rng();
    loop {
        let c = rng.random_range(1..n);
        let f = |x: u128| (mod_exp(x, 2, n) + c) % n;
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd((x as i128 - y as i128).unsigned_abs(), n);
        }
        if d != n && d != 1 {
            return d;
        }
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn factor(mut n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    if n == 1 {
        return factors;
    }
    let mut i = 3;
    let mut max_trial = cmp::min((n as f64).sqrt() as u128 + 1, 1000);
    while i <= max_trial {
        while n % i == 0 {
            factors.push(i);
            n /= i;
            max_trial = cmp::min((n as f64).sqrt() as u128 + 1, 1000);
        }
        i += 2;
    }
    if n == 1 {
        return factors;
    }
    if is_prime(n) {
        factors.push(n);
        return factors;
    }
    let d = pollards_rho(n);
    factors.extend(factor(d));
    factors.extend(factor(n / d));
    factors
}
