/// a(n) = 3*n^2 + 3*n + 4
/// https://oeis.org/A000772

pub struct A000772;

impl crate::traits::IntegerSequence for A000772 {
    const NAME: &str = "a(n) = 3*n^2 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 10, 22, 40, 64, 94, 130, 172, 220, 274, 334, 400, 472, 550, 634, 724, 820, 922, 1030, 1144, 1264, 1390, 1522, 1660, 1804, 1954, 2110, 2272, 2440, 2614
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000772";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_772(n)
    }
}

const fn quad_772(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000772>();
}
