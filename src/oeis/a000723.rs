/// a(n) = 1*n^2 + 4*n + 4
/// https://oeis.org/A000723

pub struct A000723;

impl crate::traits::IntegerSequence for A000723 {
    const NAME: &str = "a(n) = 1*n^2 + 4*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000723";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_723(n)
    }
}

const fn quad_723(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 4 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000723>();
}
