/// a(n) = n^3 + 5*n + 0
/// https://oeis.org/A000505

pub struct A000505;

impl crate::traits::IntegerSequence for A000505 {
    const NAME: &str = "a(n) = n^3 + 5*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 18, 42, 84, 150, 246, 378, 552, 774, 1050, 1386, 1788, 2262, 2814, 3450, 4176, 4998, 5922, 6954, 8100, 9366, 10758, 12282, 13944, 15750, 17706, 19818, 22092, 24534
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000505";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_505(n)
    }
}

const fn poly_505(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000505>();
}
