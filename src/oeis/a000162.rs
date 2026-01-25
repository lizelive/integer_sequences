/// a(n) = n^2 + 2*n + 1
/// https://oeis.org/A000162

pub struct A000162;

impl crate::traits::IntegerSequence for A000162 {
    const NAME: &str = "a(n) = n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000162";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_162(n)
    }
}

const fn poly_162(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000162>();
}
