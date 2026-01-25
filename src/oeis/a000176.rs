/// a(n) = n^2 + 6*n + 2
/// https://oeis.org/A000176

pub struct A000176;

impl crate::traits::IntegerSequence for A000176 {
    const NAME: &str = "a(n) = n^2 + 6*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 18, 29, 42, 57, 74, 93, 114, 137, 162, 189, 218, 249, 282, 317, 354, 393, 434, 477, 522, 569, 618, 669, 722
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000176";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_176(n)
    }
}

const fn poly_176(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 6 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000176>();
}
