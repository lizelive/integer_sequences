/// a(n) = 6*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001029

pub struct A001029;

impl crate::traits::IntegerSequence for A001029 {
    const NAME: &str = "a(n) = 6*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 11, 66, 201, 452, 855, 1446, 2261, 3336, 4707, 6410, 8481, 10956, 13871, 17262, 21165, 25616, 30651, 36306, 42617, 49620, 57351, 65846, 75141, 85272, 96275, 108186, 121041, 134876, 149727
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001029";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1029(n)
    }
}

const fn cubic_1029(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001029>();
}
