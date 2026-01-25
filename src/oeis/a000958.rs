/// a(n) = 9*n^2 + 5
/// https://oeis.org/A000958

pub struct A000958;

impl crate::traits::IntegerSequence for A000958 {
    const NAME: &str = "a(n) = 9*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 14, 41, 86, 149, 230, 329, 446, 581, 734, 905, 1094, 1301, 1526, 1769, 2030, 2309, 2606, 2921, 3254, 3605, 3974, 4361, 4766, 5189, 5630, 6089, 6566, 7061, 7574
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000958";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_958(n)
    }
}

const fn sq_958(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000958>();
}
