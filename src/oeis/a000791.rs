/// a(n) = 4*n^2 + 2*n + 3
/// https://oeis.org/A000791

pub struct A000791;

impl crate::traits::IntegerSequence for A000791 {
    const NAME: &str = "a(n) = 4*n^2 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 23, 45, 75, 113, 159, 213, 275, 345, 423, 509, 603, 705, 815, 933, 1059, 1193, 1335, 1485, 1643, 1809, 1983, 2165, 2355, 2553, 2759, 2973, 3195, 3425
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000791";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_791(n)
    }
}

const fn quad_791(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000791>();
}
