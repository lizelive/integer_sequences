/// a(n) = 1*n^2 + 3*n + 4
/// https://oeis.org/A000722

pub struct A000722;

impl crate::traits::IntegerSequence for A000722 {
    const NAME: &str = "a(n) = 1*n^2 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 14, 22, 32, 44, 58, 74, 92, 112, 134, 158, 184, 212, 242, 274, 308, 344, 382, 422, 464, 508, 554, 602, 652, 704, 758, 814, 872, 932
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000722";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_722(n)
    }
}

const fn quad_722(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000722>();
}
