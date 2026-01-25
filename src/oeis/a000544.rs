/// a(n) = n^3 + 4*n + 4
/// https://oeis.org/A000544

pub struct A000544;

impl crate::traits::IntegerSequence for A000544 {
    const NAME: &str = "a(n) = n^3 + 4*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 20, 43, 84, 149, 244, 375, 548, 769, 1044, 1379, 1780, 2253, 2804, 3439, 4164, 4985, 5908, 6939, 8084, 9349, 10740, 12263, 13924, 15729, 17684, 19795, 22068, 24509
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000544";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_544(n)
    }
}

const fn poly_544(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000544>();
}
