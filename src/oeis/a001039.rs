/// a(n) = 4*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001039

pub struct A001039;

impl crate::traits::IntegerSequence for A001039 {
    const NAME: &str = "a(n) = 4*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 40, 123, 280, 535, 912, 1435, 2128, 3015, 4120, 5467, 7080, 8983, 11200, 13755, 16672, 19975, 23688, 27835, 32440, 37527, 43120, 49243, 55920, 63175, 71032, 79515, 88648, 98455
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001039";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1039(n)
    }
}

const fn cubic_1039(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001039>();
}
