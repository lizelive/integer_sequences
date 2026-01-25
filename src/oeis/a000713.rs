/// a(n) = 1*n^2 + 4*n + 2
/// https://oeis.org/A000713

pub struct A000713;

impl crate::traits::IntegerSequence for A000713 {
    const NAME: &str = "a(n) = 1*n^2 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 14, 23, 34, 47, 62, 79, 98, 119, 142, 167, 194, 223, 254, 287, 322, 359, 398, 439, 482, 527, 574, 623, 674, 727, 782, 839, 898, 959
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000713";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_713(n)
    }
}

const fn quad_713(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000713>();
}
