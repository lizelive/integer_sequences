/// a(n) = n^3 + 6*n + 6
/// https://oeis.org/A000566

pub struct A000566;

impl crate::traits::IntegerSequence for A000566 {
    const NAME: &str = "a(n) = n^3 + 6*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 13, 26, 51, 94, 161, 258, 391, 566, 789, 1066, 1403, 1806, 2281, 2834, 3471, 4198, 5021, 5946, 6979, 8126, 9393, 10786, 12311, 13974, 15781, 17738, 19851, 22126, 24569
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000566";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_566(n)
    }
}

const fn poly_566(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000566>();
}
