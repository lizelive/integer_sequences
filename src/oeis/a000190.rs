/// a(n) = n^2 + 0*n + 4
/// https://oeis.org/A000190

pub struct A000190;

impl crate::traits::IntegerSequence for A000190 {
    const NAME: &str = "a(n) = n^2 + 0*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 5, 8, 13, 20, 29, 40, 53, 68, 85, 104, 125, 148, 173, 200, 229, 260, 293, 328, 365, 404, 445, 488, 533, 580
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000190";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_190(n)
    }
}

const fn poly_190(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 0 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000190>();
}
