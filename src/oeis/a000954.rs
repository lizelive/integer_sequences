/// a(n) = 5*n^2 + 5
/// https://oeis.org/A000954

pub struct A000954;

impl crate::traits::IntegerSequence for A000954 {
    const NAME: &str = "a(n) = 5*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 10, 25, 50, 85, 130, 185, 250, 325, 410, 505, 610, 725, 850, 985, 1130, 1285, 1450, 1625, 1810, 2005, 2210, 2425, 2650, 2885, 3130, 3385, 3650, 3925, 4210
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000954";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_954(n)
    }
}

const fn sq_954(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000954>();
}
