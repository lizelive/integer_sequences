/// a(n) = 1*n^2 + 0
/// https://oeis.org/A000900

pub struct A000900;

impl crate::traits::IntegerSequence for A000900 {
    const NAME: &str = "a(n) = 1*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000900";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_900(n)
    }
}

const fn sq_900(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000900>();
}
