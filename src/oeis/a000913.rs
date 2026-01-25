/// a(n) = 4*n^2 + 1
/// https://oeis.org/A000913

pub struct A000913;

impl crate::traits::IntegerSequence for A000913 {
    const NAME: &str = "a(n) = 4*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 17, 37, 65, 101, 145, 197, 257, 325, 401, 485, 577, 677, 785, 901, 1025, 1157, 1297, 1445, 1601, 1765, 1937, 2117, 2305, 2501, 2705, 2917, 3137, 3365
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000913";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_913(n)
    }
}

const fn sq_913(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000913>();
}
