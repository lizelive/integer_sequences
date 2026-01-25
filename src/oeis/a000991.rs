/// a(n) = 2*n^2 + 9
/// https://oeis.org/A000991

pub struct A000991;

impl crate::traits::IntegerSequence for A000991 {
    const NAME: &str = "a(n) = 2*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 11, 17, 27, 41, 59, 81, 107, 137, 171, 209, 251, 297, 347, 401, 459, 521, 587, 657, 731, 809, 891, 977, 1067, 1161, 1259, 1361, 1467, 1577, 1691
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000991";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_991(n)
    }
}

const fn sq_991(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000991>();
}
