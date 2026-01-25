/// a(n) = n^3 + 4*n^2 + 0*n + 0
/// https://oeis.org/A000204

pub struct A000204;

impl crate::traits::IntegerSequence for A000204 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 0*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 24, 63, 128, 225, 360, 539, 768, 1053, 1400, 1815, 2304, 2873, 3528, 4275, 5120, 6069, 7128, 8303, 9600, 11025, 12584, 14283, 16128
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000204";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_204(n)
    }
}

const fn poly_204(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 0 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000204>();
}
