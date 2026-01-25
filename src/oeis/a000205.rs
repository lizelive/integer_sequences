/// a(n) = n^3 + 0*n^2 + 1*n + 0
/// https://oeis.org/A000205

pub struct A000205;

impl crate::traits::IntegerSequence for A000205 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 10, 30, 68, 130, 222, 350, 520, 738, 1010, 1342, 1740, 2210, 2758, 3390, 4112, 4930, 5850, 6878, 8020, 9282, 10670, 12190, 13848
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000205";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_205(n)
    }
}

const fn poly_205(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000205>();
}
