/// Number of composite numbers <= n.
/// https://oeis.org/A000046

pub struct A000046;

impl crate::traits::IntegerSequence for A000046 {
    const NAME: &str = "Number of composite numbers <= n";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 5, 5, 6, 6, 7, 8, 9, 9, 10, 10, 11, 12, 13, 13, 14, 15, 16,
        17, 18, 18, 19, 19, 20, 21, 22, 23, 24, 24, 25, 26, 27, 27, 28, 28, 29, 30, 31, 31, 32, 33,
        34, 35, 36, 36, 37, 38, 39, 40, 41, 41, 42, 42, 43, 44, 45, 46, 47, 47, 48, 49, 50, 50, 51,
        51, 52, 53, 54, 55, 56, 56, 57, 58, 59, 59, 60, 61, 62, 63, 64, 64, 65, 66, 67, 68, 69, 70,
        71, 71, 72, 73,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000046";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        count_composites(n)
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

const fn count_composites(n: crate::Index) -> crate::Value {
    if n < 4 {
        return 0;
    }
    let mut count = 0;
    let mut i = 4;
    while i <= n {
        if !is_prime(i) {
            count += 1;
        }
        i += 1;
    }
    count
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000046>();
}
