/// a(n) = 6*n^2 + 5
/// https://oeis.org/A000955

pub struct A000955;

impl crate::traits::IntegerSequence for A000955 {
    const NAME: &str = "a(n) = 6*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 11, 29, 59, 101, 155, 221, 299, 389, 491, 605, 731, 869, 1019, 1181, 1355, 1541, 1739, 1949, 2171, 2405, 2651, 2909, 3179, 3461, 3755, 4061, 4379, 4709, 5051
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000955";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_955(n)
    }
}

const fn sq_955(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000955>();
}
