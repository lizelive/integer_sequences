/// a(n) = n^3 + 2*n + 3
/// https://oeis.org/A000532

pub struct A000532;

impl crate::traits::IntegerSequence for A000532 {
    const NAME: &str = "a(n) = n^3 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 15, 36, 75, 138, 231, 360, 531, 750, 1023, 1356, 1755, 2226, 2775, 3408, 4131, 4950, 5871, 6900, 8043, 9306, 10695, 12216, 13875, 15678, 17631, 19740, 22011, 24450
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000532";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_532(n)
    }
}

const fn poly_532(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000532>();
}
