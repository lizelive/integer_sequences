/// a(n) = n^3 + 1*n^2 + 0*n + 1
/// https://oeis.org/A000226

pub struct A000226;

impl crate::traits::IntegerSequence for A000226 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 13, 37, 81, 151, 253, 393, 577, 811, 1101, 1453, 1873, 2367, 2941, 3601, 4353, 5203, 6157, 7221, 8401, 9703, 11133, 12697, 14401
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000226";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_226(n)
    }
}

const fn poly_226(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000226>();
}
