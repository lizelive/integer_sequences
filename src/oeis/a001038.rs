/// a(n) = 3*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001038

pub struct A001038;

impl crate::traits::IntegerSequence for A001038 {
    const NAME: &str = "a(n) = 3*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 32, 96, 216, 410, 696, 1092, 1616, 2286, 3120, 4136, 5352, 6786, 8456, 10380, 12576, 15062, 17856, 20976, 24440, 28266, 32472, 37076, 42096, 47550, 53456, 59832, 66696, 74066
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001038";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1038(n)
    }
}

const fn cubic_1038(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001038>();
}
