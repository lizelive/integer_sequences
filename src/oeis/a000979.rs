/// a(n) = 10*n^2 + 7
/// https://oeis.org/A000979

pub struct A000979;

impl crate::traits::IntegerSequence for A000979 {
    const NAME: &str = "a(n) = 10*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 17, 47, 97, 167, 257, 367, 497, 647, 817, 1007, 1217, 1447, 1697, 1967, 2257, 2567, 2897, 3247, 3617, 4007, 4417, 4847, 5297, 5767, 6257, 6767, 7297, 7847, 8417
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000979";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_979(n)
    }
}

const fn sq_979(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000979>();
}
