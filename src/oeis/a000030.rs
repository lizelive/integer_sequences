/// Initial digit of n.
/// https://oeis.org/A000030

pub struct A000030;

impl crate::traits::IntegerSequence for A000030 {
    const NAME: &str = "Initial digit of n";

    const HEAD: &[crate::Value] = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000030";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        initial_digit(n)
    }
}

const fn initial_digit(n: crate::Index) -> crate::Value {
    if n < 0 {
        return initial_digit(-n);
    }
    let mut m = n;
    while m >= 10 {
        m /= 10;
    }
    m
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000030>();
}
