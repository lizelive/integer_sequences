/// a(n) = n*(n+3)/2.
/// https://oeis.org/A000091

pub struct A000091;

impl crate::traits::IntegerSequence for A000091 {
    const NAME: &str = "a(n) = prime(n) - 2";

    const HEAD: &[crate::Value] = &[
        0, 1, 3, 5, 9, 11, 15, 17, 21, 27, 29, 35, 39, 41, 45, 51, 57, 59, 65, 69, 71, 77, 81, 87,
        95, 99, 101, 105, 107, 111, 125, 129, 135, 137, 147, 149, 155, 161, 165, 171, 177, 179,
        189, 191, 195, 197, 209, 221, 225, 227, 231, 237, 239, 249, 255, 261, 267, 269, 275, 279,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000091";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        nth_prime(n) - 2
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

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000091>();
}
