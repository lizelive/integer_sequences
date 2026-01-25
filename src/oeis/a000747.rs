/// a(n) = 2*n^2 + 3*n + 4
/// https://oeis.org/A000747

pub struct A000747;

impl crate::traits::IntegerSequence for A000747 {
    const NAME: &str = "a(n) = 2*n^2 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 18, 31, 48, 69, 94, 123, 156, 193, 234, 279, 328, 381, 438, 499, 564, 633, 706, 783, 864, 949, 1038, 1131, 1228, 1329, 1434, 1543, 1656, 1773
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000747";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_747(n)
    }
}

const fn quad_747(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000747>();
}
