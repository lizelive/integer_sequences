/// a(n) = 4*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001003

pub struct A001003;

impl crate::traits::IntegerSequence for A001003 {
    const NAME: &str = "a(n) = 4*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 34, 111, 260, 505, 870, 1379, 2056, 2925, 4010, 5335, 6924, 8801, 10990, 13515, 16400, 19669, 23346, 27455, 32020, 37065, 42614, 48691, 55320, 62525, 70330, 78759, 87836, 97585
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001003";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1003(n)
    }
}

const fn cubic_1003(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001003>();
}
