/// a(n) = 4*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001009

pub struct A001009;

impl crate::traits::IntegerSequence for A001009 {
    const NAME: &str = "a(n) = 4*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 38, 120, 276, 530, 906, 1428, 2120, 3006, 4110, 5456, 7068, 8970, 11186, 13740, 16656, 19958, 23670, 27816, 32420, 37506, 43098, 49220, 55896, 63150, 71006, 79488, 88620, 98426
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001009";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1009(n)
    }
}

const fn cubic_1009(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001009>();
}
