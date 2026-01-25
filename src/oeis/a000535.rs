/// a(n) = n^3 + 5*n + 3
/// https://oeis.org/A000535

pub struct A000535;

impl crate::traits::IntegerSequence for A000535 {
    const NAME: &str = "a(n) = n^3 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 21, 45, 87, 153, 249, 381, 555, 777, 1053, 1389, 1791, 2265, 2817, 3453, 4179, 5001, 5925, 6957, 8103, 9369, 10761, 12285, 13947, 15753, 17709, 19821, 22095, 24537
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000535";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_535(n)
    }
}

const fn poly_535(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000535>();
}
