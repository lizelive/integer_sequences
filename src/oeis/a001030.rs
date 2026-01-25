/// a(n) = 1*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001030

pub struct A001030;

impl crate::traits::IntegerSequence for A001030 {
    const NAME: &str = "a(n) = 1*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 3, 12, 33, 72, 135, 228, 357, 528, 747, 1020, 1353, 1752, 2223, 2772, 3405, 4128, 4947, 5868, 6897, 8040, 9303, 10692, 12213, 13872, 15675, 17628, 19737, 22008, 24447
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001030";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1030(n)
    }
}

const fn cubic_1030(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001030>();
}
