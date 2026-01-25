/// a(n) = n^3 + 1*n + 0
/// https://oeis.org/A000501

pub struct A000501;

impl crate::traits::IntegerSequence for A000501 {
    const NAME: &str = "a(n) = n^3 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 10, 30, 68, 130, 222, 350, 520, 738, 1010, 1342, 1740, 2210, 2758, 3390, 4112, 4930, 5850, 6878, 8020, 9282, 10670, 12190, 13848, 15650, 17602, 19710, 21980, 24418
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000501";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_501(n)
    }
}

const fn poly_501(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000501>();
}
