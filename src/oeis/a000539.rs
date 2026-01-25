/// a(n) = n^3 + 9*n + 3
/// https://oeis.org/A000539

pub struct A000539;

impl crate::traits::IntegerSequence for A000539 {
    const NAME: &str = "a(n) = n^3 + 9*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 13, 29, 57, 103, 173, 273, 409, 587, 813, 1093, 1433, 1839, 2317, 2873, 3513, 4243, 5069, 5997, 7033, 8183, 9453, 10849, 12377, 14043, 15853, 17813, 19929, 22207, 24653
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000539";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_539(n)
    }
}

const fn poly_539(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000539>();
}
