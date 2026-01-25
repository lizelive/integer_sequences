/// a(n) = 5*T(n) + 9
/// https://oeis.org/A000399

pub struct A000399;

impl crate::traits::IntegerSequence for A000399 {
    const NAME: &str = "a(n) = 5*T(n) + 9";

    const HEAD: &[crate::Value] = &[
        9, 14, 24, 39, 59, 84, 114, 149, 189, 234, 284, 339, 399, 464, 534, 609, 689, 774, 864, 959, 1059, 1164, 1274, 1389, 1509
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000399";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_399(n)
    }
}

const fn tri_399(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000399>();
}
