/// a(n) = 3*n^2 + 8
/// https://oeis.org/A000982

pub struct A000982;

impl crate::traits::IntegerSequence for A000982 {
    const NAME: &str = "a(n) = 3*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 11, 20, 35, 56, 83, 116, 155, 200, 251, 308, 371, 440, 515, 596, 683, 776, 875, 980, 1091, 1208, 1331, 1460, 1595, 1736, 1883, 2036, 2195, 2360, 2531
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000982";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_982(n)
    }
}

const fn sq_982(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000982>();
}
