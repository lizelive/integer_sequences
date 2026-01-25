/// a(n) = n^3 + 3*n^2 + 2*n + 1
/// https://oeis.org/A000238

pub struct A000238;

impl crate::traits::IntegerSequence for A000238 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 25, 61, 121, 211, 337, 505, 721, 991, 1321, 1717, 2185, 2731, 3361, 4081, 4897, 5815, 6841, 7981, 9241, 10627, 12145, 13801, 15601
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000238";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_238(n)
    }
}

const fn poly_238(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000238>();
}
