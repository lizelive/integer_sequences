/// a(n) = n^3 + 8*n + 6
/// https://oeis.org/A000568

pub struct A000568;

impl crate::traits::IntegerSequence for A000568 {
    const NAME: &str = "a(n) = n^3 + 8*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 15, 30, 57, 102, 171, 270, 405, 582, 807, 1086, 1425, 1830, 2307, 2862, 3501, 4230, 5055, 5982, 7017, 8166, 9435, 10830, 12357, 14022, 15831, 17790, 19905, 22182, 24627
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000568";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_568(n)
    }
}

const fn poly_568(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000568>();
}
