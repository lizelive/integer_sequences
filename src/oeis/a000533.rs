/// a(n) = n^3 + 3*n + 3
/// https://oeis.org/A000533

pub struct A000533;

impl crate::traits::IntegerSequence for A000533 {
    const NAME: &str = "a(n) = n^3 + 3*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 7, 17, 39, 79, 143, 237, 367, 539, 759, 1033, 1367, 1767, 2239, 2789, 3423, 4147, 4967, 5889, 6919, 8063, 9327, 10717, 12239, 13899, 15703, 17657, 19767, 22039, 24479
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000533";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_533(n)
    }
}

const fn poly_533(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000533>();
}
