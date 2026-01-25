/// a(n) = n^3 + 8*n + 0
/// https://oeis.org/A000508

pub struct A000508;

impl crate::traits::IntegerSequence for A000508 {
    const NAME: &str = "a(n) = n^3 + 8*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 9, 24, 51, 96, 165, 264, 399, 576, 801, 1080, 1419, 1824, 2301, 2856, 3495, 4224, 5049, 5976, 7011, 8160, 9429, 10824, 12351, 14016, 15825, 17784, 19899, 22176, 24621
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000508";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_508(n)
    }
}

const fn poly_508(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000508>();
}
