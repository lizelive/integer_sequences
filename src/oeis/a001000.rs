/// a(n) = 1*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001000

pub struct A001000;

impl crate::traits::IntegerSequence for A001000 {
    const NAME: &str = "a(n) = 1*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 2, 10, 30, 68, 130, 222, 350, 520, 738, 1010, 1342, 1740, 2210, 2758, 3390, 4112, 4930, 5850, 6878, 8020, 9282, 10670, 12190, 13848, 15650, 17602, 19710, 21980, 24418
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001000";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1000(n)
    }
}

const fn cubic_1000(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001000>();
}
