/// a(n) = 2*n^2 + 8
/// https://oeis.org/A000981

pub struct A000981;

impl crate::traits::IntegerSequence for A000981 {
    const NAME: &str = "a(n) = 2*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 10, 16, 26, 40, 58, 80, 106, 136, 170, 208, 250, 296, 346, 400, 458, 520, 586, 656, 730, 808, 890, 976, 1066, 1160, 1258, 1360, 1466, 1576, 1690
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000981";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_981(n)
    }
}

const fn sq_981(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000981>();
}
