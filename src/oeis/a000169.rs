/// a(n) = n^2 + 9*n + 1
/// https://oeis.org/A000169

pub struct A000169;

impl crate::traits::IntegerSequence for A000169 {
    const NAME: &str = "a(n) = n^2 + 9*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 23, 37, 53, 71, 91, 113, 137, 163, 191, 221, 253, 287, 323, 361, 401, 443, 487, 533, 581, 631, 683, 737, 793
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000169";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_169(n)
    }
}

const fn poly_169(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 9 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000169>();
}
