/// a(n) = floor(log_2(n)).
/// https://oeis.org/A000024

pub struct A000024;

impl crate::traits::IntegerSequence for A000024 {
    const NAME: &str = "a(n) = floor(log_2(n))";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000024";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        floor_log2(n)
    }
}

const fn floor_log2(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut result = 0;
    let mut m = n;
    while m > 1 {
        m /= 2;
        result += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000024>();
}
