/// a(n) = n^3 + 7*n + 6
/// https://oeis.org/A000567

pub struct A000567;

impl crate::traits::IntegerSequence for A000567 {
    const NAME: &str = "a(n) = n^3 + 7*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 14, 28, 54, 98, 166, 264, 398, 574, 798, 1076, 1414, 1818, 2294, 2848, 3486, 4214, 5038, 5964, 6998, 8146, 9414, 10808, 12334, 13998, 15806, 17764, 19878, 22154, 24598
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000567";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_567(n)
    }
}

const fn poly_567(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000567>();
}
