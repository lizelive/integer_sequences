/// a(n) = n^3 + 1*n^2 + 3*n + 1
/// https://oeis.org/A000241

pub struct A000241;

impl crate::traits::IntegerSequence for A000241 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 19, 46, 93, 166, 271, 414, 601, 838, 1131, 1486, 1909, 2406, 2983, 3646, 4401, 5254, 6211, 7278, 8461, 9766, 11199, 12766, 14473
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000241";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_241(n)
    }
}

const fn poly_241(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000241>();
}
