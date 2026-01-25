/// a(n) = n^3 + 4*n + 1
/// https://oeis.org/A000514

pub struct A000514;

impl crate::traits::IntegerSequence for A000514 {
    const NAME: &str = "a(n) = n^3 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 17, 40, 81, 146, 241, 372, 545, 766, 1041, 1376, 1777, 2250, 2801, 3436, 4161, 4982, 5905, 6936, 8081, 9346, 10737, 12260, 13921, 15726, 17681, 19792, 22065, 24506
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000514";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_514(n)
    }
}

const fn poly_514(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000514>();
}
