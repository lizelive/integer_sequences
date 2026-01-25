/// a(n) = binomial(n+4, 5).
/// https://oeis.org/A000094

pub struct A000094;

impl crate::traits::IntegerSequence for A000094 {
    const NAME: &str = "Largest prime less than n";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 2, 3, 3, 5, 5, 7, 7, 7, 11, 11, 13, 13, 13, 13, 17, 17, 19, 19, 19, 19, 23, 23,
        23, 23, 23, 29, 29, 31, 31, 31, 31, 31, 31, 37, 37, 37, 37, 41, 41, 43, 43, 43, 43, 47, 47,
        47, 47, 47, 47, 53, 53, 53, 53, 53, 53, 59, 59, 61, 61, 61, 61, 61, 61, 67, 67, 67, 67, 71,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000094";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        largest_prime_less_than(n)
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

const fn largest_prime_less_than(n: crate::Index) -> crate::Value {
    if n <= 2 {
        return 0;
    }
    let mut k = n - 1;
    while k >= 2 {
        if is_prime(k) {
            return k;
        }
        k -= 1;
    }
    0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000094>();
}
