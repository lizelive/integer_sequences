/// a(n) = n^2 + 5*n + 1
/// https://oeis.org/A000165

pub struct A000165;

impl crate::traits::IntegerSequence for A000165 {
    const NAME: &str = "a(n) = n^2 + 5*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 15, 25, 37, 51, 67, 85, 105, 127, 151, 177, 205, 235, 267, 301, 337, 375, 415, 457, 501, 547, 595, 645, 697
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000165";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_165(n)
    }
}

const fn poly_165(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 5 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000165>();
}
