/// a(n) = 5*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001022

pub struct A001022;

impl crate::traits::IntegerSequence for A001022 {
    const NAME: &str = "a(n) = 5*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 54, 165, 372, 705, 1194, 1869, 2760, 3897, 5310, 7029, 9084, 11505, 14322, 17565, 21264, 25449, 30150, 35397, 41220, 47649, 54714, 62445, 70872, 80025, 89934, 100629, 112140, 124497
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001022";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1022(n)
    }
}

const fn cubic_1022(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001022>();
}
