/// a(n) = 8*n^2 + 5
/// https://oeis.org/A000957

pub struct A000957;

impl crate::traits::IntegerSequence for A000957 {
    const NAME: &str = "a(n) = 8*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 13, 37, 77, 133, 205, 293, 397, 517, 653, 805, 973, 1157, 1357, 1573, 1805, 2053, 2317, 2597, 2893, 3205, 3533, 3877, 4237, 4613, 5005, 5413, 5837, 6277, 6733
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000957";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_957(n)
    }
}

const fn sq_957(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000957>();
}
