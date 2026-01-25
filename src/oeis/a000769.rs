/// a(n) = 3*n^2 + 5*n + 3
/// https://oeis.org/A000769

pub struct A000769;

impl crate::traits::IntegerSequence for A000769 {
    const NAME: &str = "a(n) = 3*n^2 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 11, 25, 45, 71, 103, 141, 185, 235, 291, 353, 421, 495, 575, 661, 753, 851, 955, 1065, 1181, 1303, 1431, 1565, 1705, 1851, 2003, 2161, 2325, 2495, 2671
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000769";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_769(n)
    }
}

const fn quad_769(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000769>();
}
