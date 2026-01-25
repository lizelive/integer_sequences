/// a(n) = n^3 + 2*n + 8
/// https://oeis.org/A000582

pub struct A000582;

impl crate::traits::IntegerSequence for A000582 {
    const NAME: &str = "a(n) = n^3 + 2*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 11, 20, 41, 80, 143, 236, 365, 536, 755, 1028, 1361, 1760, 2231, 2780, 3413, 4136, 4955, 5876, 6905, 8048, 9311, 10700, 12221, 13880, 15683, 17636, 19745, 22016, 24455
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000582";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_582(n)
    }
}

const fn poly_582(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000582>();
}
