/// a(n) = 5*n^2 + 1
/// https://oeis.org/A000914

pub struct A000914;

impl crate::traits::IntegerSequence for A000914 {
    const NAME: &str = "a(n) = 5*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 21, 46, 81, 126, 181, 246, 321, 406, 501, 606, 721, 846, 981, 1126, 1281, 1446, 1621, 1806, 2001, 2206, 2421, 2646, 2881, 3126, 3381, 3646, 3921, 4206
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000914";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_914(n)
    }
}

const fn sq_914(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000914>();
}
