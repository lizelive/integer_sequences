/// a(n) = n^3 + 3*n + 7
/// https://oeis.org/A000573

pub struct A000573;

impl crate::traits::IntegerSequence for A000573 {
    const NAME: &str = "a(n) = n^3 + 3*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 11, 21, 43, 83, 147, 241, 371, 543, 763, 1037, 1371, 1771, 2243, 2793, 3427, 4151, 4971, 5893, 6923, 8067, 9331, 10721, 12243, 13903, 15707, 17661, 19771, 22043, 24483
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000573";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_573(n)
    }
}

const fn poly_573(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000573>();
}
