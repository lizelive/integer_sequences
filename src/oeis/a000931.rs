/// a(n) = 2*n^2 + 3
/// https://oeis.org/A000931

pub struct A000931;

impl crate::traits::IntegerSequence for A000931 {
    const NAME: &str = "a(n) = 2*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 5, 11, 21, 35, 53, 75, 101, 131, 165, 203, 245, 291, 341, 395, 453, 515, 581, 651, 725, 803, 885, 971, 1061, 1155, 1253, 1355, 1461, 1571, 1685
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000931";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_931(n)
    }
}

const fn sq_931(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000931>();
}
