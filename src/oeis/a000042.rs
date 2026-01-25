/// Unary representation of natural numbers.
/// https://oeis.org/A000042

pub struct A000042;

impl crate::traits::IntegerSequence for A000042 {
    const NAME: &str = "Unary representation of natural numbers";

    const HEAD: &[crate::Value] = &[
        1, 11, 111, 1111, 11111, 111111, 1111111, 11111111, 111111111, 1111111111, 11111111111,
        111111111111, 1111111111111, 11111111111111, 111111111111111, 1111111111111111,
        11111111111111111, 111111111111111111,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000042";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        repunit(n)
    }
}

const fn repunit(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    // R_n = (10^n - 1) / 9
    let mut result = 0isize;
    let mut i = 0;
    while i < n {
        result = result * 10 + 1;
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000042>();
}
