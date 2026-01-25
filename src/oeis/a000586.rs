/// a(n) = n^3 + 6*n + 8
/// https://oeis.org/A000586

pub struct A000586;

impl crate::traits::IntegerSequence for A000586 {
    const NAME: &str = "a(n) = n^3 + 6*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 15, 28, 53, 96, 163, 260, 393, 568, 791, 1068, 1405, 1808, 2283, 2836, 3473, 4200, 5023, 5948, 6981, 8128, 9395, 10788, 12313, 13976, 15783, 17740, 19853, 22128, 24571
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000586";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_586(n)
    }
}

const fn poly_586(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000586>();
}
