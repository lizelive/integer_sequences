/// a(n) = 1*n^2 + 4
/// https://oeis.org/A000940

pub struct A000940;

impl crate::traits::IntegerSequence for A000940 {
    const NAME: &str = "a(n) = 1*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 5, 8, 13, 20, 29, 40, 53, 68, 85, 104, 125, 148, 173, 200, 229, 260, 293, 328, 365, 404, 445, 488, 533, 580, 629, 680, 733, 788, 845
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000940";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_940(n)
    }
}

const fn sq_940(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000940>();
}
