/// a(n) = 4*n^2 + 2*n + 0
/// https://oeis.org/A000776

pub struct A000776;

impl crate::traits::IntegerSequence for A000776 {
    const NAME: &str = "a(n) = 4*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 20, 42, 72, 110, 156, 210, 272, 342, 420, 506, 600, 702, 812, 930, 1056, 1190, 1332, 1482, 1640, 1806, 1980, 2162, 2352, 2550, 2756, 2970, 3192, 3422
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000776";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_776(n)
    }
}

const fn quad_776(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000776>();
}
