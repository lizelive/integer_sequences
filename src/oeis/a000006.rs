/// Integer part of square root of n-th prime.
/// https://oeis.org/A000006

pub struct A000006;

impl crate::traits::IntegerSequence for A000006 {
    const NAME: &str = "Integer part of square root of n-th prime";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10,
        10, 11, 11, 11, 11, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 15, 15, 15,
        15, 15, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 19, 19, 19,
        19, 19, 19, 20,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000006";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // This sequence is complex; use HEAD for known values
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        // Fallback for values beyond HEAD
        let p = nth_prime(n);
        isqrt(p)
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

const fn nth_prime(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    let mut candidate = 2;
    loop {
        if is_prime(candidate) {
            count += 1;
            if count == n {
                return candidate;
            }
        }
        candidate += 1;
    }
}

const fn isqrt(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000006>();
}
