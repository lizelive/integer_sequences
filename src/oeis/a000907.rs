/// a(n) = 8*n^2 + 0
/// https://oeis.org/A000907

pub struct A000907;

impl crate::traits::IntegerSequence for A000907 {
    const NAME: &str = "a(n) = 8*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 32, 72, 128, 200, 288, 392, 512, 648, 800, 968, 1152, 1352, 1568, 1800, 2048, 2312, 2592, 2888, 3200, 3528, 3872, 4232, 4608, 5000, 5408, 5832, 6272, 6728
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000907";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_907(n)
    }
}

const fn sq_907(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000907>();
}
