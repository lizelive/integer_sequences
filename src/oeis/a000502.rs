/// a(n) = n^3 + 2*n + 0
/// https://oeis.org/A000502

pub struct A000502;

impl crate::traits::IntegerSequence for A000502 {
    const NAME: &str = "a(n) = n^3 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 12, 33, 72, 135, 228, 357, 528, 747, 1020, 1353, 1752, 2223, 2772, 3405, 4128, 4947, 5868, 6897, 8040, 9303, 10692, 12213, 13872, 15675, 17628, 19737, 22008, 24447
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000502";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_502(n)
    }
}

const fn poly_502(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000502>();
}
