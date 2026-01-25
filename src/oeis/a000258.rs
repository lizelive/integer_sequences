/// a(n) = 9*n^2
/// https://oeis.org/A000258

pub struct A000258;

impl crate::traits::IntegerSequence for A000258 {
    const NAME: &str = "a(n) = 9*n^2";

    const HEAD: &[crate::Value] = &[
        0, 9, 36, 81, 144, 225, 324, 441, 576, 729, 900, 1089, 1296, 1521, 1764, 2025, 2304, 2601, 2916, 3249, 3600, 3969, 4356, 4761, 5184
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000258";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_258(n)
    }
}

const fn power_258(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000258>();
}
