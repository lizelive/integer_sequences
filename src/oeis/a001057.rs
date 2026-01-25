/// a(n) = 4*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001057

pub struct A001057;

impl crate::traits::IntegerSequence for A001057 {
    const NAME: &str = "a(n) = 4*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 10, 52, 150, 328, 610, 1020, 1582, 2320, 3258, 4420, 5830, 7512, 9490, 11788, 14430, 17440, 20842, 24660, 28918, 33640, 38850, 44572, 50830, 57648, 65050, 73060, 81702, 91000, 100978
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001057";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1057(n)
    }
}

const fn cubic_1057(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001057>();
}
