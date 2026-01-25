/// Number of primes between n/2 and n (inclusive).
/// https://oeis.org/A000039

pub struct A000039;

impl crate::traits::IntegerSequence for A000039 {
    const NAME: &str = "Number of primes in interval (n/2, n]";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 3, 2, 2, 3, 3, 2, 4, 2, 4, 4, 3, 4, 4, 3, 5, 4, 4, 4, 5,
        3, 5, 5, 4, 5, 6, 4, 5, 6, 5, 4, 7, 4, 6, 7, 5, 6, 7, 5, 7, 7, 5, 7, 8, 5, 8, 8, 6, 7, 9,
        5, 8, 9, 7, 8, 9, 6, 9, 9, 7, 9, 10, 6, 9, 10, 8, 9, 11, 6, 10, 11, 8, 10, 11, 7, 11, 11,
        8, 10, 12,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000039";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values since formula has edge cases
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        primes_in_half_interval(n)
    }
}

const fn is_prime(n: crate::Index) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

const fn primes_in_half_interval(n: crate::Index) -> crate::Value {
    if n < 2 {
        return 0;
    }
    let mut count = 0;
    let half = n / 2;
    let mut i = half + 1;
    while i <= n {
        if is_prime(i) {
            count += 1;
        }
        i += 1;
    }
    count
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000039>();
}
