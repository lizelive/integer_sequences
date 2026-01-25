/// a(n) = 2*T(n) + 3
/// https://oeis.org/A000363

pub struct A000363;

impl crate::traits::IntegerSequence for A000363 {
    const NAME: &str = "a(n) = 2*T(n) + 3";

    const HEAD: &[crate::Value] = &[
        3, 5, 9, 15, 23, 33, 45, 59, 75, 93, 113, 135, 159, 185, 213, 243, 275, 309, 345, 383, 423, 465, 509, 555, 603
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000363";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_363(n)
    }
}

const fn tri_363(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000363>();
}
