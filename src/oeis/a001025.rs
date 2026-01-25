/// a(n) = 2*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001025

pub struct A001025;

impl crate::traits::IntegerSequence for A001025 {
    const NAME: &str = "a(n) = 2*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 34, 93, 196, 355, 582, 889, 1288, 1791, 2410, 3157, 4044, 5083, 6286, 7665, 9232, 10999, 12978, 15181, 17620, 20307, 23254, 26473, 29976, 33775, 37882, 42309, 47068, 52171
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001025";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1025(n)
    }
}

const fn cubic_1025(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001025>();
}
