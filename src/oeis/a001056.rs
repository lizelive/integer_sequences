/// a(n) = 3*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001056

pub struct A001056;

impl crate::traits::IntegerSequence for A001056 {
    const NAME: &str = "a(n) = 3*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 44, 123, 264, 485, 804, 1239, 1808, 2529, 3420, 4499, 5784, 7293, 9044, 11055, 13344, 15929, 18828, 22059, 25640, 29589, 33924, 38663, 43824, 49425, 55484, 62019, 69048, 76589
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001056";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1056(n)
    }
}

const fn cubic_1056(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001056>();
}
