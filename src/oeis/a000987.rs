/// a(n) = 8*n^2 + 8
/// https://oeis.org/A000987

pub struct A000987;

impl crate::traits::IntegerSequence for A000987 {
    const NAME: &str = "a(n) = 8*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 16, 40, 80, 136, 208, 296, 400, 520, 656, 808, 976, 1160, 1360, 1576, 1808, 2056, 2320, 2600, 2896, 3208, 3536, 3880, 4240, 4616, 5008, 5416, 5840, 6280, 6736
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000987";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_987(n)
    }
}

const fn sq_987(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000987>();
}
