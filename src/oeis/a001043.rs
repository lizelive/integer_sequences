/// a(n) = 2*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001043

pub struct A001043;

impl crate::traits::IntegerSequence for A001043 {
    const NAME: &str = "a(n) = 2*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 28, 78, 168, 310, 516, 798, 1168, 1638, 2220, 2926, 3768, 4758, 5908, 7230, 8736, 10438, 12348, 14478, 16840, 19446, 22308, 25438, 28848, 32550, 36556, 40878, 45528, 50518
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001043";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1043(n)
    }
}

const fn cubic_1043(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001043>();
}
