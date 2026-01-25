/// a(n) = n^3 + 2*n^2 + 4*n + 0
/// https://oeis.org/A000222

pub struct A000222;

impl crate::traits::IntegerSequence for A000222 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 24, 57, 112, 195, 312, 469, 672, 927, 1240, 1617, 2064, 2587, 3192, 3885, 4672, 5559, 6552, 7657, 8880, 10227, 11704, 13317, 15072
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000222";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_222(n)
    }
}

const fn poly_222(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000222>();
}
