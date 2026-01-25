/// a(n) = 2*T(n) + 5
/// https://oeis.org/A000365

pub struct A000365;

impl crate::traits::IntegerSequence for A000365 {
    const NAME: &str = "a(n) = 2*T(n) + 5";

    const HEAD: &[crate::Value] = &[
        5, 7, 11, 17, 25, 35, 47, 61, 77, 95, 115, 137, 161, 187, 215, 245, 277, 311, 347, 385, 425, 467, 511, 557, 605
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000365";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_365(n)
    }
}

const fn tri_365(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000365>();
}
