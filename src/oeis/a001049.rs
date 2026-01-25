/// a(n) = 2*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001049

pub struct A001049;

impl crate::traits::IntegerSequence for A001049 {
    const NAME: &str = "a(n) = 2*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 32, 87, 184, 335, 552, 847, 1232, 1719, 2320, 3047, 3912, 4927, 6104, 7455, 8992, 10727, 12672, 14839, 17240, 19887, 22792, 25967, 29424, 33175, 37232, 41607, 46312, 51359
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001049";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1049(n)
    }
}

const fn cubic_1049(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001049>();
}
