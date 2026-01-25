/// a(n) = 6*n^3 + 2*n^2 + 1*n
/// https://oeis.org/A001017

pub struct A001017;

impl crate::traits::IntegerSequence for A001017 {
    const NAME: &str = "a(n) = 6*n^3 + 2*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 58, 183, 420, 805, 1374, 2163, 3208, 4545, 6210, 8239, 10668, 13533, 16870, 20715, 25104, 30073, 35658, 41895, 48820, 56469, 64878, 74083, 84120, 95025, 106834, 119583, 133308, 148045
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001017";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1017(n)
    }
}

const fn cubic_1017(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 2 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001017>();
}
