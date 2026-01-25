/// a(n) = n^2 + 5*n + 3
/// https://oeis.org/A000185

pub struct A000185;

impl crate::traits::IntegerSequence for A000185 {
    const NAME: &str = "a(n) = n^2 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 17, 27, 39, 53, 69, 87, 107, 129, 153, 179, 207, 237, 269, 303, 339, 377, 417, 459, 503, 549, 597, 647, 699
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000185";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_185(n)
    }
}

const fn poly_185(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000185>();
}
