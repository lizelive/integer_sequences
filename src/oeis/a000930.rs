/// a(n) = 1*n^2 + 3
/// https://oeis.org/A000930

pub struct A000930;

impl crate::traits::IntegerSequence for A000930 {
    const NAME: &str = "a(n) = 1*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 4, 7, 12, 19, 28, 39, 52, 67, 84, 103, 124, 147, 172, 199, 228, 259, 292, 327, 364, 403, 444, 487, 532, 579, 628, 679, 732, 787, 844
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000930";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_930(n)
    }
}

const fn sq_930(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000930>();
}
