/// a(n) = n^3 + 2*n^2 + 1*n + 0
/// https://oeis.org/A000207

pub struct A000207;

impl crate::traits::IntegerSequence for A000207 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 18, 48, 100, 180, 294, 448, 648, 900, 1210, 1584, 2028, 2548, 3150, 3840, 4624, 5508, 6498, 7600, 8820, 10164, 11638, 13248, 15000
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000207";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_207(n)
    }
}

const fn poly_207(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000207>();
}
