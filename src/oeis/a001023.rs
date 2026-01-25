/// a(n) = 6*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001023

pub struct A001023;

impl crate::traits::IntegerSequence for A001023 {
    const NAME: &str = "a(n) = 6*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 10, 62, 192, 436, 830, 1410, 2212, 3272, 4626, 6310, 8360, 10812, 13702, 17066, 20940, 25360, 30362, 35982, 42256, 49220, 56910, 65362, 74612, 84696, 95650, 107510, 120312, 134092, 148886
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001023";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1023(n)
    }
}

const fn cubic_1023(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001023>();
}
