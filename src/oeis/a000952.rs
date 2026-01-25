/// a(n) = 3*n^2 + 5
/// https://oeis.org/A000952

pub struct A000952;

impl crate::traits::IntegerSequence for A000952 {
    const NAME: &str = "a(n) = 3*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 8, 17, 32, 53, 80, 113, 152, 197, 248, 305, 368, 437, 512, 593, 680, 773, 872, 977, 1088, 1205, 1328, 1457, 1592, 1733, 1880, 2033, 2192, 2357, 2528
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000952";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_952(n)
    }
}

const fn sq_952(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000952>();
}
