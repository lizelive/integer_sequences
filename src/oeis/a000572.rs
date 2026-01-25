/// a(n) = n^3 + 2*n + 7
/// https://oeis.org/A000572

pub struct A000572;

impl crate::traits::IntegerSequence for A000572 {
    const NAME: &str = "a(n) = n^3 + 2*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 10, 19, 40, 79, 142, 235, 364, 535, 754, 1027, 1360, 1759, 2230, 2779, 3412, 4135, 4954, 5875, 6904, 8047, 9310, 10699, 12220, 13879, 15682, 17635, 19744, 22015, 24454
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000572";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_572(n)
    }
}

const fn poly_572(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000572>();
}
