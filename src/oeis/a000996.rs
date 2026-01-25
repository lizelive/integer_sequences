/// a(n) = 7*n^2 + 9
/// https://oeis.org/A000996

pub struct A000996;

impl crate::traits::IntegerSequence for A000996 {
    const NAME: &str = "a(n) = 7*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 16, 37, 72, 121, 184, 261, 352, 457, 576, 709, 856, 1017, 1192, 1381, 1584, 1801, 2032, 2277, 2536, 2809, 3096, 3397, 3712, 4041, 4384, 4741, 5112, 5497, 5896
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000996";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_996(n)
    }
}

const fn sq_996(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000996>();
}
