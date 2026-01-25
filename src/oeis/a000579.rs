/// a(n) = n^3 + 9*n + 7
/// https://oeis.org/A000579

pub struct A000579;

impl crate::traits::IntegerSequence for A000579 {
    const NAME: &str = "a(n) = n^3 + 9*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 17, 33, 61, 107, 177, 277, 413, 591, 817, 1097, 1437, 1843, 2321, 2877, 3517, 4247, 5073, 6001, 7037, 8187, 9457, 10853, 12381, 14047, 15857, 17817, 19933, 22211, 24657
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000579";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_579(n)
    }
}

const fn poly_579(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000579>();
}
