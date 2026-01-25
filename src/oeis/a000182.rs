/// a(n) = n^2 + 2*n + 3
/// https://oeis.org/A000182

pub struct A000182;

impl crate::traits::IntegerSequence for A000182 {
    const NAME: &str = "a(n) = n^2 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 11, 18, 27, 38, 51, 66, 83, 102, 123, 146, 171, 198, 227, 258, 291, 326, 363, 402, 443, 486, 531, 578, 627
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000182";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_182(n)
    }
}

const fn poly_182(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000182>();
}
