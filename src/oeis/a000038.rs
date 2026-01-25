/// Twice A000217(n), n-th triangular number: a(n) = n*(n+1).
/// https://oeis.org/A000038

pub struct A000038;

impl crate::traits::IntegerSequence for A000038 {
    const NAME: &str = "Twice the triangular numbers: a(n) = n*(n+1)";

    const HEAD: &[crate::Value] = &[
        0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182, 210, 240, 272, 306, 342, 380, 420,
        462, 506, 552, 600, 650, 702, 756, 812, 870, 930, 992, 1056, 1122, 1190, 1260, 1332, 1406,
        1482, 1560, 1640, 1722, 1806, 1892, 1980, 2070, 2162, 2256, 2352, 2450, 2550, 2652, 2756,
        2862, 2970, 3080, 3192, 3306, 3422, 3540, 3660, 3782, 3906, 4032, 4160, 4290, 4422, 4556,
        4692, 4830, 4970, 5112, 5256, 5402, 5550, 5700, 5852, 6006, 6162, 6320, 6480, 6642, 6806,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000038";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        n * (n + 1)
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000038>();
}
