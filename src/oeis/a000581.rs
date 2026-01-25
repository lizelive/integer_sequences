/// a(n) = n^3 + 1*n + 8
/// https://oeis.org/A000581

pub struct A000581;

impl crate::traits::IntegerSequence for A000581 {
    const NAME: &str = "a(n) = n^3 + 1*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 10, 18, 38, 76, 138, 230, 358, 528, 746, 1018, 1350, 1748, 2218, 2766, 3398, 4120, 4938, 5858, 6886, 8028, 9290, 10678, 12198, 13856, 15658, 17610, 19718, 21988, 24426
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000581";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_581(n)
    }
}

const fn poly_581(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000581>();
}
