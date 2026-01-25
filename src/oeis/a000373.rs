/// a(n) = 3*T(n) + 3
/// https://oeis.org/A000373

pub struct A000373;

impl crate::traits::IntegerSequence for A000373 {
    const NAME: &str = "a(n) = 3*T(n) + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 12, 21, 33, 48, 66, 87, 111, 138, 168, 201, 237, 276, 318, 363, 411, 462, 516, 573, 633, 696, 762, 831, 903
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000373";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_373(n)
    }
}

const fn tri_373(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000373>();
}
