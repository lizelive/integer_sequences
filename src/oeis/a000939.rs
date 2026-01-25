/// a(n) = 10*n^2 + 3
/// https://oeis.org/A000939

pub struct A000939;

impl crate::traits::IntegerSequence for A000939 {
    const NAME: &str = "a(n) = 10*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 13, 43, 93, 163, 253, 363, 493, 643, 813, 1003, 1213, 1443, 1693, 1963, 2253, 2563, 2893, 3243, 3613, 4003, 4413, 4843, 5293, 5763, 6253, 6763, 7293, 7843, 8413
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000939";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_939(n)
    }
}

const fn sq_939(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000939>();
}
