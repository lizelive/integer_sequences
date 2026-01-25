/// a(n) = 4*n^2 + 2*n + 2
/// https://oeis.org/A000786

pub struct A000786;

impl crate::traits::IntegerSequence for A000786 {
    const NAME: &str = "a(n) = 4*n^2 + 2*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 22, 44, 74, 112, 158, 212, 274, 344, 422, 508, 602, 704, 814, 932, 1058, 1192, 1334, 1484, 1642, 1808, 1982, 2164, 2354, 2552, 2758, 2972, 3194, 3424
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000786";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_786(n)
    }
}

const fn quad_786(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000786>();
}
