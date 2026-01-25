/// d(n) (also called tau(n) or sigma_0(n)), the number of divisors of n.
/// https://oeis.org/A000005

pub struct A000005;

impl crate::traits::IntegerSequence for A000005 {
    const NAME: &str = "d(n) (also called tau(n) or sigma_0(n)), the number of divisors of n";

    const HEAD: &[crate::Value] = &[
        1, 2, 2, 3, 2, 4, 2, 4, 3, 4, 2, 6, 2, 4, 4, 5, 2, 6, 2, 6, 4, 4, 2, 8, 3, 4, 4, 6, 2, 8,
        2, 6, 4, 4, 4, 9, 2, 4, 4, 8, 2, 8, 2, 6, 6, 4, 2, 10, 3, 6, 4, 6, 2, 8, 4, 8, 4, 4, 2, 12,
        2, 4, 6, 7, 4, 8, 2, 6, 4, 8, 2, 12, 2, 4, 6, 6, 4, 8, 2, 10, 5, 4, 2, 12, 4, 4, 4, 8, 2,
        12, 4, 6, 4, 4, 4, 12, 2, 6, 6, 9,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000005";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        divisor_count(n)
    }
}

const fn divisor_count(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
        i += 1;
    }
    count
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000005>();
}
