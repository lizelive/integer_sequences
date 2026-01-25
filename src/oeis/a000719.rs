/// a(n) = 1*n^2 + 5*n + 3
/// https://oeis.org/A000719

pub struct A000719;

impl crate::traits::IntegerSequence for A000719 {
    const NAME: &str = "a(n) = 1*n^2 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 17, 27, 39, 53, 69, 87, 107, 129, 153, 179, 207, 237, 269, 303, 339, 377, 417, 459, 503, 549, 597, 647, 699, 753, 809, 867, 927, 989
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000719";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_719(n)
    }
}

const fn quad_719(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000719>();
}
