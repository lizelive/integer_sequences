/// a(n) = 1*n^2 + 8
/// https://oeis.org/A000980

pub struct A000980;

impl crate::traits::IntegerSequence for A000980 {
    const NAME: &str = "a(n) = 1*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 9, 12, 17, 24, 33, 44, 57, 72, 89, 108, 129, 152, 177, 204, 233, 264, 297, 332, 369, 408, 449, 492, 537, 584, 633, 684, 737, 792, 849
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000980";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_980(n)
    }
}

const fn sq_980(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000980>();
}
