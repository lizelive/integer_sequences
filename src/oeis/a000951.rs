/// a(n) = 2*n^2 + 5
/// https://oeis.org/A000951

pub struct A000951;

impl crate::traits::IntegerSequence for A000951 {
    const NAME: &str = "a(n) = 2*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 7, 13, 23, 37, 55, 77, 103, 133, 167, 205, 247, 293, 343, 397, 455, 517, 583, 653, 727, 805, 887, 973, 1063, 1157, 1255, 1357, 1463, 1573, 1687
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000951";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_951(n)
    }
}

const fn sq_951(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000951>();
}
