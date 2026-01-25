/// a(n) = n^3 + 0*n + 4
/// https://oeis.org/A000540

pub struct A000540;

impl crate::traits::IntegerSequence for A000540 {
    const NAME: &str = "a(n) = n^3 + 0*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 5, 12, 31, 68, 129, 220, 347, 516, 733, 1004, 1335, 1732, 2201, 2748, 3379, 4100, 4917, 5836, 6863, 8004, 9265, 10652, 12171, 13828, 15629, 17580, 19687, 21956, 24393
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000540";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_540(n)
    }
}

const fn poly_540(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000540>();
}
