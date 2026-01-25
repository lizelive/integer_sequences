/// a(n) = 3*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001002

pub struct A001002;

impl crate::traits::IntegerSequence for A001002 {
    const NAME: &str = "a(n) = 3*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 4, 26, 84, 196, 380, 654, 1036, 1544, 2196, 3010, 4004, 5196, 6604, 8246, 10140, 12304, 14756, 17514, 20596, 24020, 27804, 31966, 36524, 41496, 46900, 52754, 59076, 65884, 73196
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001002";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1002(n)
    }
}

const fn cubic_1002(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001002>();
}
