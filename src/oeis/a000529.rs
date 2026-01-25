/// a(n) = n^3 + 9*n + 2
/// https://oeis.org/A000529

pub struct A000529;

impl crate::traits::IntegerSequence for A000529 {
    const NAME: &str = "a(n) = n^3 + 9*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 12, 28, 56, 102, 172, 272, 408, 586, 812, 1092, 1432, 1838, 2316, 2872, 3512, 4242, 5068, 5996, 7032, 8182, 9452, 10848, 12376, 14042, 15852, 17812, 19928, 22206, 24652
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000529";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_529(n)
    }
}

const fn poly_529(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000529>();
}
