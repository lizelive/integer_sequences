/// a(n) = n^2 + 8*n + 1
/// https://oeis.org/A000168

pub struct A000168;

impl crate::traits::IntegerSequence for A000168 {
    const NAME: &str = "a(n) = n^2 + 8*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 21, 34, 49, 66, 85, 106, 129, 154, 181, 210, 241, 274, 309, 346, 385, 426, 469, 514, 561, 610, 661, 714, 769
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000168";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_168(n)
    }
}

const fn poly_168(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 8 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000168>();
}
