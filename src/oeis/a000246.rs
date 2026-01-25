/// a(n) = n^3 + 1*n^2 + 4*n + 1
/// https://oeis.org/A000246

pub struct A000246;

impl crate::traits::IntegerSequence for A000246 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 21, 49, 97, 171, 277, 421, 609, 847, 1141, 1497, 1921, 2419, 2997, 3661, 4417, 5271, 6229, 7297, 8481, 9787, 11221, 12789, 14497
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000246";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_246(n)
    }
}

const fn poly_246(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000246>();
}
