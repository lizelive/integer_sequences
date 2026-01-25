/// a(n) = n^3 + 1*n + 1
/// https://oeis.org/A000511

pub struct A000511;

impl crate::traits::IntegerSequence for A000511 {
    const NAME: &str = "a(n) = n^3 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 11, 31, 69, 131, 223, 351, 521, 739, 1011, 1343, 1741, 2211, 2759, 3391, 4113, 4931, 5851, 6879, 8021, 9283, 10671, 12191, 13849, 15651, 17603, 19711, 21981, 24419
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000511";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_511(n)
    }
}

const fn poly_511(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000511>();
}
