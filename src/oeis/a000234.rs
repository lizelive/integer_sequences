/// a(n) = n^3 + 4*n^2 + 1*n + 1
/// https://oeis.org/A000234

pub struct A000234;

impl crate::traits::IntegerSequence for A000234 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 27, 67, 133, 231, 367, 547, 777, 1063, 1411, 1827, 2317, 2887, 3543, 4291, 5137, 6087, 7147, 8323, 9621, 11047, 12607, 14307, 16153
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000234";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_234(n)
    }
}

const fn poly_234(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000234>();
}
