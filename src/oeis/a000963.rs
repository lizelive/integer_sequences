/// a(n) = 4*n^2 + 6
/// https://oeis.org/A000963

pub struct A000963;

impl crate::traits::IntegerSequence for A000963 {
    const NAME: &str = "a(n) = 4*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 10, 22, 42, 70, 106, 150, 202, 262, 330, 406, 490, 582, 682, 790, 906, 1030, 1162, 1302, 1450, 1606, 1770, 1942, 2122, 2310, 2506, 2710, 2922, 3142, 3370
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000963";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_963(n)
    }
}

const fn sq_963(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000963>();
}
