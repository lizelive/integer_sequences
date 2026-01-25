/// a(n) = 2*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001055

pub struct A001055;

impl crate::traits::IntegerSequence for A001055 {
    const NAME: &str = "a(n) = 2*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 36, 96, 200, 360, 588, 896, 1296, 1800, 2420, 3168, 4056, 5096, 6300, 7680, 9248, 11016, 12996, 15200, 17640, 20328, 23276, 26496, 30000, 33800, 37908, 42336, 47096, 52200
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001055";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1055(n)
    }
}

const fn cubic_1055(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001055>();
}
