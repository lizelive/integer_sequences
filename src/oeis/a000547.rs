/// a(n) = n^3 + 7*n + 4
/// https://oeis.org/A000547

pub struct A000547;

impl crate::traits::IntegerSequence for A000547 {
    const NAME: &str = "a(n) = n^3 + 7*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 12, 26, 52, 96, 164, 262, 396, 572, 796, 1074, 1412, 1816, 2292, 2846, 3484, 4212, 5036, 5962, 6996, 8144, 9412, 10806, 12332, 13996, 15804, 17762, 19876, 22152, 24596
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000547";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_547(n)
    }
}

const fn poly_547(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000547>();
}
