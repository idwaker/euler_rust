// check if given number is prime
fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    let upto = (n as f64).sqrt() as i64;
    for i in 3..upto {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

struct Primes {
    curr: i64
}

// ieterator for primes
impl Iterator for Primes {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {

        let mut next = self.curr;
        while !is_prime(next) {
            next += 1;
        }
        self.curr = next + 1;
        Some(next)
    }
}

fn factor(val: i64) -> i64 {
    let primes = Primes {curr: 1};
    if is_prime(val) { return val; }
    for i in primes {
        let quot = val as f64 / i as f64;
        if val % i == 0 {
            // so we have a factor
            return factor(quot as i64);
        }
    }
    // we always have to return i64
    // above will not satisfy the condition because last return is
    // inside if condition
    return 0;
}

fn main() {
    let value:i64 = 600851475143;

    // find largest prime factor
    println!("Factor: {}", factor(value));
}
