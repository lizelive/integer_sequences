/// a(n) = 5*n^2 + 7
/// https://oeis.org/A000974

pub struct A000974;

impl crate::traits::IntegerSequence for A000974 {
    const NAME: &str = "a(n) = 5*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 12, 27, 52, 87, 132, 187, 252, 327, 412, 507, 612, 727, 852, 987, 1132, 1287, 1452, 1627, 1812, 2007, 2212, 2427, 2652, 2887, 3132, 3387, 3652, 3927, 4212
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000974";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_974(n)
    }
}

const fn sq_974(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000974>();
}
