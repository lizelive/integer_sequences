/// a(n) = n^3 + 8*n + 3
/// https://oeis.org/A000538

pub struct A000538;

impl crate::traits::IntegerSequence for A000538 {
    const NAME: &str = "a(n) = n^3 + 8*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 12, 27, 54, 99, 168, 267, 402, 579, 804, 1083, 1422, 1827, 2304, 2859, 3498, 4227, 5052, 5979, 7014, 8163, 9432, 10827, 12354, 14019, 15828, 17787, 19902, 22179, 24624
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000538";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_538(n)
    }
}

const fn poly_538(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000538>();
}
