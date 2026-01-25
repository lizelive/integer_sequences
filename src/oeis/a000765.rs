/// a(n) = 3*n^2 + 1*n + 3
/// https://oeis.org/A000765

pub struct A000765;

impl crate::traits::IntegerSequence for A000765 {
    const NAME: &str = "a(n) = 3*n^2 + 1*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 7, 17, 33, 55, 83, 117, 157, 203, 255, 313, 377, 447, 523, 605, 693, 787, 887, 993, 1105, 1223, 1347, 1477, 1613, 1755, 1903, 2057, 2217, 2383, 2555
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000765";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_765(n)
    }
}

const fn quad_765(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000765>();
}
