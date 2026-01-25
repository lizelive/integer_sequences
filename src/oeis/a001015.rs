/// a(n) = 4*n^3 + 2*n^2 + 1*n
/// https://oeis.org/A001015

pub struct A001015;

impl crate::traits::IntegerSequence for A001015 {
    const NAME: &str = "a(n) = 4*n^3 + 2*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 42, 129, 292, 555, 942, 1477, 2184, 3087, 4210, 5577, 7212, 9139, 11382, 13965, 16912, 20247, 23994, 28177, 32820, 37947, 43582, 49749, 56472, 63775, 71682, 80217, 89404, 99267
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001015";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1015(n)
    }
}

const fn cubic_1015(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 2 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001015>();
}
