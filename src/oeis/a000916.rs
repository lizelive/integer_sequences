/// a(n) = 7*n^2 + 1
/// https://oeis.org/A000916

pub struct A000916;

impl crate::traits::IntegerSequence for A000916 {
    const NAME: &str = "a(n) = 7*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 29, 64, 113, 176, 253, 344, 449, 568, 701, 848, 1009, 1184, 1373, 1576, 1793, 2024, 2269, 2528, 2801, 3088, 3389, 3704, 4033, 4376, 4733, 5104, 5489, 5888
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000916";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_916(n)
    }
}

const fn sq_916(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000916>();
}
