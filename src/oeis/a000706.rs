/// a(n) = 1*n^2 + 2*n + 1
/// https://oeis.org/A000706

pub struct A000706;

impl crate::traits::IntegerSequence for A000706 {
    const NAME: &str = "a(n) = 1*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000706";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_706(n)
    }
}

const fn quad_706(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000706>();
}
