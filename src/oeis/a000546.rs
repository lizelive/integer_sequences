/// a(n) = n^3 + 6*n + 4
/// https://oeis.org/A000546

pub struct A000546;

impl crate::traits::IntegerSequence for A000546 {
    const NAME: &str = "a(n) = n^3 + 6*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 24, 49, 92, 159, 256, 389, 564, 787, 1064, 1401, 1804, 2279, 2832, 3469, 4196, 5019, 5944, 6977, 8124, 9391, 10784, 12309, 13972, 15779, 17736, 19849, 22124, 24567
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000546";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_546(n)
    }
}

const fn poly_546(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000546>();
}
