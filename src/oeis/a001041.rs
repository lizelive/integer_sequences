/// a(n) = 6*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001041

pub struct A001041;

impl crate::traits::IntegerSequence for A001041 {
    const NAME: &str = "a(n) = 6*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 56, 177, 408, 785, 1344, 2121, 3152, 4473, 6120, 8129, 10536, 13377, 16688, 20505, 24864, 29801, 35352, 41553, 48440, 56049, 64416, 73577, 83568, 94425, 106184, 118881, 132552, 147233
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001041";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1041(n)
    }
}

const fn cubic_1041(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001041>();
}
