/// a(n) = n^3 + 3*n^2 + 1*n + 1
/// https://oeis.org/A000233

pub struct A000233;

impl crate::traits::IntegerSequence for A000233 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 23, 58, 117, 206, 331, 498, 713, 982, 1311, 1706, 2173, 2718, 3347, 4066, 4881, 5798, 6823, 7962, 9221, 10606, 12123, 13778, 15577
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000233";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_233(n)
    }
}

const fn poly_233(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000233>();
}
