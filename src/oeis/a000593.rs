/// a(n) = n^3 + 3*n + 9
/// https://oeis.org/A000593

pub struct A000593;

impl crate::traits::IntegerSequence for A000593 {
    const NAME: &str = "a(n) = n^3 + 3*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 13, 23, 45, 85, 149, 243, 373, 545, 765, 1039, 1373, 1773, 2245, 2795, 3429, 4153, 4973, 5895, 6925, 8069, 9333, 10723, 12245, 13905, 15709, 17663, 19773, 22045, 24485
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000593";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_593(n)
    }
}

const fn poly_593(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000593>();
}
