/// a(n) = n^3 + 3*n + 4
/// https://oeis.org/A000543

pub struct A000543;

impl crate::traits::IntegerSequence for A000543 {
    const NAME: &str = "a(n) = n^3 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 18, 40, 80, 144, 238, 368, 540, 760, 1034, 1368, 1768, 2240, 2790, 3424, 4148, 4968, 5890, 6920, 8064, 9328, 10718, 12240, 13900, 15704, 17658, 19768, 22040, 24480
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000543";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_543(n)
    }
}

const fn poly_543(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000543>();
}
