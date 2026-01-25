/// a(n) = n^3 + 7*n + 5
/// https://oeis.org/A000557

pub struct A000557;

impl crate::traits::IntegerSequence for A000557 {
    const NAME: &str = "a(n) = n^3 + 7*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 13, 27, 53, 97, 165, 263, 397, 573, 797, 1075, 1413, 1817, 2293, 2847, 3485, 4213, 5037, 5963, 6997, 8145, 9413, 10807, 12333, 13997, 15805, 17763, 19877, 22153, 24597
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000557";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_557(n)
    }
}

const fn poly_557(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000557>();
}
