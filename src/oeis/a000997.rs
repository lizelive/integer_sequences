/// a(n) = 8*n^2 + 9
/// https://oeis.org/A000997

pub struct A000997;

impl crate::traits::IntegerSequence for A000997 {
    const NAME: &str = "a(n) = 8*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 17, 41, 81, 137, 209, 297, 401, 521, 657, 809, 977, 1161, 1361, 1577, 1809, 2057, 2321, 2601, 2897, 3209, 3537, 3881, 4241, 4617, 5009, 5417, 5841, 6281, 6737
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000997";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_997(n)
    }
}

const fn sq_997(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000997>();
}
