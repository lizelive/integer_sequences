/// a(n) = 1*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001006

pub struct A001006;

impl crate::traits::IntegerSequence for A001006 {
    const NAME: &str = "a(n) = 1*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 3, 14, 39, 84, 155, 258, 399, 584, 819, 1110, 1463, 1884, 2379, 2954, 3615, 4368, 5219, 6174, 7239, 8420, 9723, 11154, 12719, 14424, 16275, 18278, 20439, 22764, 25259
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001006";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1006(n)
    }
}

const fn cubic_1006(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001006>();
}
