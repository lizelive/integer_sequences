/// a(n) = 1*n^2 + 1*n + 1
/// https://oeis.org/A000705

pub struct A000705;

impl crate::traits::IntegerSequence for A000705 {
    const NAME: &str = "a(n) = 1*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 7, 13, 21, 31, 43, 57, 73, 91, 111, 133, 157, 183, 211, 241, 273, 307, 343, 381, 421, 463, 507, 553, 601, 651, 703, 757, 813, 871
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000705";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_705(n)
    }
}

const fn quad_705(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000705>();
}
