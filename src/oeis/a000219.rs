/// a(n) = n^3 + 4*n^2 + 3*n + 0
/// https://oeis.org/A000219

pub struct A000219;

impl crate::traits::IntegerSequence for A000219 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 30, 72, 140, 240, 378, 560, 792, 1080, 1430, 1848, 2340, 2912, 3570, 4320, 5168, 6120, 7182, 8360, 9660, 11088, 12650, 14352, 16200
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000219";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_219(n)
    }
}

const fn poly_219(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000219>();
}
