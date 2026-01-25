/// a(n) = n^3 + 3*n + 8
/// https://oeis.org/A000583

pub struct A000583;

impl crate::traits::IntegerSequence for A000583 {
    const NAME: &str = "a(n) = n^3 + 3*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 12, 22, 44, 84, 148, 242, 372, 544, 764, 1038, 1372, 1772, 2244, 2794, 3428, 4152, 4972, 5894, 6924, 8068, 9332, 10722, 12244, 13904, 15708, 17662, 19772, 22044, 24484
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000583";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_583(n)
    }
}

const fn poly_583(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000583>();
}
