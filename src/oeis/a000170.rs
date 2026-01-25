/// a(n) = n^2 + 0*n + 2
/// https://oeis.org/A000170

pub struct A000170;

impl crate::traits::IntegerSequence for A000170 {
    const NAME: &str = "a(n) = n^2 + 0*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 3, 6, 11, 18, 27, 38, 51, 66, 83, 102, 123, 146, 171, 198, 227, 258, 291, 326, 363, 402, 443, 486, 531, 578
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000170";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_170(n)
    }
}

const fn poly_170(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 0 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000170>();
}
