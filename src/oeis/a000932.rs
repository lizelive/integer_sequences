/// a(n) = 3*n^2 + 3
/// https://oeis.org/A000932

pub struct A000932;

impl crate::traits::IntegerSequence for A000932 {
    const NAME: &str = "a(n) = 3*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 15, 30, 51, 78, 111, 150, 195, 246, 303, 366, 435, 510, 591, 678, 771, 870, 975, 1086, 1203, 1326, 1455, 1590, 1731, 1878, 2031, 2190, 2355, 2526
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000932";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_932(n)
    }
}

const fn sq_932(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000932>();
}
