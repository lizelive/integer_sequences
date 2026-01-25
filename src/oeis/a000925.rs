/// a(n) = 6*n^2 + 2
/// https://oeis.org/A000925

pub struct A000925;

impl crate::traits::IntegerSequence for A000925 {
    const NAME: &str = "a(n) = 6*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 26, 56, 98, 152, 218, 296, 386, 488, 602, 728, 866, 1016, 1178, 1352, 1538, 1736, 1946, 2168, 2402, 2648, 2906, 3176, 3458, 3752, 4058, 4376, 4706, 5048
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000925";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_925(n)
    }
}

const fn sq_925(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000925>();
}
