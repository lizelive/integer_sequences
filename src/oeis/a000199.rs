/// a(n) = n^2 + 9*n + 4
/// https://oeis.org/A000199

pub struct A000199;

impl crate::traits::IntegerSequence for A000199 {
    const NAME: &str = "a(n) = n^2 + 9*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 14, 26, 40, 56, 74, 94, 116, 140, 166, 194, 224, 256, 290, 326, 364, 404, 446, 490, 536, 584, 634, 686, 740, 796
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000199";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_199(n)
    }
}

const fn poly_199(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 9 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000199>();
}
