/// a(n) = n^3 + 9*n + 9
/// https://oeis.org/A000599

pub struct A000599;

impl crate::traits::IntegerSequence for A000599 {
    const NAME: &str = "a(n) = n^3 + 9*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 19, 35, 63, 109, 179, 279, 415, 593, 819, 1099, 1439, 1845, 2323, 2879, 3519, 4249, 5075, 6003, 7039, 8189, 9459, 10855, 12383, 14049, 15859, 17819, 19935, 22213, 24659
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000599";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_599(n)
    }
}

const fn poly_599(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000599>();
}
