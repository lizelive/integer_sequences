/// a(n) = 1*n^2 + 1*n + 3
/// https://oeis.org/A000715

pub struct A000715;

impl crate::traits::IntegerSequence for A000715 {
    const NAME: &str = "a(n) = 1*n^2 + 1*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 5, 9, 15, 23, 33, 45, 59, 75, 93, 113, 135, 159, 185, 213, 243, 275, 309, 345, 383, 423, 465, 509, 555, 603, 653, 705, 759, 815, 873
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000715";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_715(n)
    }
}

const fn quad_715(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 1 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000715>();
}
