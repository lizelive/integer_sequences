/// a(n) = n^3 + 1*n^2 + 1*n + 0
/// https://oeis.org/A000206

pub struct A000206;

impl crate::traits::IntegerSequence for A000206 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 14, 39, 84, 155, 258, 399, 584, 819, 1110, 1463, 1884, 2379, 2954, 3615, 4368, 5219, 6174, 7239, 8420, 9723, 11154, 12719, 14424
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000206";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_206(n)
    }
}

const fn poly_206(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000206>();
}
