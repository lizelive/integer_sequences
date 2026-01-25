/// a(n) = 3*n^2 + 4*n + 2
/// https://oeis.org/A000763

pub struct A000763;

impl crate::traits::IntegerSequence for A000763 {
    const NAME: &str = "a(n) = 3*n^2 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 22, 41, 66, 97, 134, 177, 226, 281, 342, 409, 482, 561, 646, 737, 834, 937, 1046, 1161, 1282, 1409, 1542, 1681, 1826, 1977, 2134, 2297, 2466, 2641
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000763";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_763(n)
    }
}

const fn quad_763(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000763>();
}
