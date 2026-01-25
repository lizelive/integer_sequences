/// a(n) = n^3 + 2*n^2 + 4*n + 1
/// https://oeis.org/A000247

pub struct A000247;

impl crate::traits::IntegerSequence for A000247 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 25, 58, 113, 196, 313, 470, 673, 928, 1241, 1618, 2065, 2588, 3193, 3886, 4673, 5560, 6553, 7658, 8881, 10228, 11705, 13318, 15073
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000247";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_247(n)
    }
}

const fn poly_247(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000247>();
}
