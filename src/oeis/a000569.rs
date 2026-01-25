/// a(n) = n^3 + 9*n + 6
/// https://oeis.org/A000569

pub struct A000569;

impl crate::traits::IntegerSequence for A000569 {
    const NAME: &str = "a(n) = n^3 + 9*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 16, 32, 60, 106, 176, 276, 412, 590, 816, 1096, 1436, 1842, 2320, 2876, 3516, 4246, 5072, 6000, 7036, 8186, 9456, 10852, 12380, 14046, 15856, 17816, 19932, 22210, 24656
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000569";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_569(n)
    }
}

const fn poly_569(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000569>();
}
