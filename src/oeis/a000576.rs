/// a(n) = n^3 + 6*n + 7
/// https://oeis.org/A000576

pub struct A000576;

impl crate::traits::IntegerSequence for A000576 {
    const NAME: &str = "a(n) = n^3 + 6*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 14, 27, 52, 95, 162, 259, 392, 567, 790, 1067, 1404, 1807, 2282, 2835, 3472, 4199, 5022, 5947, 6980, 8127, 9394, 10787, 12312, 13975, 15782, 17739, 19852, 22127, 24570
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000576";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_576(n)
    }
}

const fn poly_576(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000576>();
}
