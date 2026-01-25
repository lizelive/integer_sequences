/// a(n) = 4*n^2 + 3*n + 1
/// https://oeis.org/A000782

pub struct A000782;

impl crate::traits::IntegerSequence for A000782 {
    const NAME: &str = "a(n) = 4*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 23, 46, 77, 116, 163, 218, 281, 352, 431, 518, 613, 716, 827, 946, 1073, 1208, 1351, 1502, 1661, 1828, 2003, 2186, 2377, 2576, 2783, 2998, 3221, 3452
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000782";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_782(n)
    }
}

const fn quad_782(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000782>();
}
