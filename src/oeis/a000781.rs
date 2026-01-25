/// a(n) = 4*n^2 + 2*n + 1
/// https://oeis.org/A000781

pub struct A000781;

impl crate::traits::IntegerSequence for A000781 {
    const NAME: &str = "a(n) = 4*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 21, 43, 73, 111, 157, 211, 273, 343, 421, 507, 601, 703, 813, 931, 1057, 1191, 1333, 1483, 1641, 1807, 1981, 2163, 2353, 2551, 2757, 2971, 3193, 3423
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000781";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_781(n)
    }
}

const fn quad_781(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000781>();
}
