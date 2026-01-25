/// a(n) = n^3 + 4*n + 7
/// https://oeis.org/A000574

pub struct A000574;

impl crate::traits::IntegerSequence for A000574 {
    const NAME: &str = "a(n) = n^3 + 4*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 12, 23, 46, 87, 152, 247, 378, 551, 772, 1047, 1382, 1783, 2256, 2807, 3442, 4167, 4988, 5911, 6942, 8087, 9352, 10743, 12266, 13927, 15732, 17687, 19798, 22071, 24512
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000574";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_574(n)
    }
}

const fn poly_574(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000574>();
}
