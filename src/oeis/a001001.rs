/// a(n) = 2*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001001

pub struct A001001;

impl crate::traits::IntegerSequence for A001001 {
    const NAME: &str = "a(n) = 2*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 3, 18, 57, 132, 255, 438, 693, 1032, 1467, 2010, 2673, 3468, 4407, 5502, 6765, 8208, 9843, 11682, 13737, 16020, 18543, 21318, 24357, 27672, 31275, 35178, 39393, 43932, 48807
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001001";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1001(n)
    }
}

const fn cubic_1001(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001001>();
}
