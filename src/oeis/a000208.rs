/// a(n) = n^3 + 3*n^2 + 1*n + 0
/// https://oeis.org/A000208

pub struct A000208;

impl crate::traits::IntegerSequence for A000208 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 22, 57, 116, 205, 330, 497, 712, 981, 1310, 1705, 2172, 2717, 3346, 4065, 4880, 5797, 6822, 7961, 9220, 10605, 12122, 13777, 15576
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000208";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_208(n)
    }
}

const fn poly_208(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000208>();
}
