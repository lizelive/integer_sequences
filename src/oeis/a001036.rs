/// a(n) = 1*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001036

pub struct A001036;

impl crate::traits::IntegerSequence for A001036 {
    const NAME: &str = "a(n) = 1*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 4, 16, 42, 88, 160, 264, 406, 592, 828, 1120, 1474, 1896, 2392, 2968, 3630, 4384, 5236, 6192, 7258, 8440, 9744, 11176, 12742, 14448, 16300, 18304, 20466, 22792, 25288
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001036";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1036(n)
    }
}

const fn cubic_1036(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001036>();
}
