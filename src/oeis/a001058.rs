/// a(n) = 5*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001058

pub struct A001058;

impl crate::traits::IntegerSequence for A001058 {
    const NAME: &str = "a(n) = 5*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 11, 60, 177, 392, 735, 1236, 1925, 2832, 3987, 5420, 7161, 9240, 11687, 14532, 17805, 21536, 25755, 30492, 35777, 41640, 48111, 55220, 62997, 71472, 80675, 90636, 101385, 112952, 125367
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001058";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1058(n)
    }
}

const fn cubic_1058(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001058>();
}
