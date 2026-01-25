/// a(n) = 4*n^2 + 0
/// https://oeis.org/A000903

pub struct A000903;

impl crate::traits::IntegerSequence for A000903 {
    const NAME: &str = "a(n) = 4*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 16, 36, 64, 100, 144, 196, 256, 324, 400, 484, 576, 676, 784, 900, 1024, 1156, 1296, 1444, 1600, 1764, 1936, 2116, 2304, 2500, 2704, 2916, 3136, 3364
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000903";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_903(n)
    }
}

const fn sq_903(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000903>();
}
