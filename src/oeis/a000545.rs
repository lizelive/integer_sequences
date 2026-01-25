/// a(n) = n^3 + 5*n + 4
/// https://oeis.org/A000545

pub struct A000545;

impl crate::traits::IntegerSequence for A000545 {
    const NAME: &str = "a(n) = n^3 + 5*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 10, 22, 46, 88, 154, 250, 382, 556, 778, 1054, 1390, 1792, 2266, 2818, 3454, 4180, 5002, 5926, 6958, 8104, 9370, 10762, 12286, 13948, 15754, 17710, 19822, 22096, 24538
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000545";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_545(n)
    }
}

const fn poly_545(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000545>();
}
