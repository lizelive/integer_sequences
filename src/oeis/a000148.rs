/// a(n) = 2*n^2 - n = n*(2n-1)
/// https://oeis.org/A000148

pub struct A000148;

impl crate::traits::IntegerSequence for A000148 {
    const NAME: &str = "a(n) = 2*n^2 - n";

    const HEAD: &[crate::Value] = &[
        0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630, 703, 780, 861, 946, 1035, 1128
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000148";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        twice_n_sq_minus_n(n)
    }
}

const fn twice_n_sq_minus_n(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n - n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000148>();
}
