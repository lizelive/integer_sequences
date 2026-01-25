/// a(n) = 3*n^2 + 6
/// https://oeis.org/A000962

pub struct A000962;

impl crate::traits::IntegerSequence for A000962 {
    const NAME: &str = "a(n) = 3*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 9, 18, 33, 54, 81, 114, 153, 198, 249, 306, 369, 438, 513, 594, 681, 774, 873, 978, 1089, 1206, 1329, 1458, 1593, 1734, 1881, 2034, 2193, 2358, 2529
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000962";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_962(n)
    }
}

const fn sq_962(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000962>();
}
