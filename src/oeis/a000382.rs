/// a(n) = 4*T(n) + 2
/// https://oeis.org/A000382

pub struct A000382;

impl crate::traits::IntegerSequence for A000382 {
    const NAME: &str = "a(n) = 4*T(n) + 2";

    const HEAD: &[crate::Value] = &[
        2, 6, 14, 26, 42, 62, 86, 114, 146, 182, 222, 266, 314, 366, 422, 482, 546, 614, 686, 762, 842, 926, 1014, 1106, 1202
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000382";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_382(n)
    }
}

const fn tri_382(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000382>();
}
