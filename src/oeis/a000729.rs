/// a(n) = 2*n^2 + 5*n + 0
/// https://oeis.org/A000729

pub struct A000729;

impl crate::traits::IntegerSequence for A000729 {
    const NAME: &str = "a(n) = 2*n^2 + 5*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 18, 33, 52, 75, 102, 133, 168, 207, 250, 297, 348, 403, 462, 525, 592, 663, 738, 817, 900, 987, 1078, 1173, 1272, 1375, 1482, 1593, 1708, 1827
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000729";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_729(n)
    }
}

const fn quad_729(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000729>();
}
