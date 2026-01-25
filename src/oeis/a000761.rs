/// a(n) = 3*n^2 + 2*n + 2
/// https://oeis.org/A000761

pub struct A000761;

impl crate::traits::IntegerSequence for A000761 {
    const NAME: &str = "a(n) = 3*n^2 + 2*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 18, 35, 58, 87, 122, 163, 210, 263, 322, 387, 458, 535, 618, 707, 802, 903, 1010, 1123, 1242, 1367, 1498, 1635, 1778, 1927, 2082, 2243, 2410, 2583
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000761";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_761(n)
    }
}

const fn quad_761(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000761>();
}
