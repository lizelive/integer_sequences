/// a(n) = 4*n^2 + 1*n + 3
/// https://oeis.org/A000790

pub struct A000790;

impl crate::traits::IntegerSequence for A000790 {
    const NAME: &str = "a(n) = 4*n^2 + 1*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 21, 42, 71, 108, 153, 206, 267, 336, 413, 498, 591, 692, 801, 918, 1043, 1176, 1317, 1466, 1623, 1788, 1961, 2142, 2331, 2528, 2733, 2946, 3167, 3396
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000790";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_790(n)
    }
}

const fn quad_790(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000790>();
}
