/// a(n) = 2*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001007

pub struct A001007;

impl crate::traits::IntegerSequence for A001007 {
    const NAME: &str = "a(n) = 2*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 4, 22, 66, 148, 280, 474, 742, 1096, 1548, 2110, 2794, 3612, 4576, 5698, 6990, 8464, 10132, 12006, 14098, 16420, 18984, 21802, 24886, 28248, 31900, 35854, 40122, 44716, 49648
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001007";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1007(n)
    }
}

const fn cubic_1007(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001007>();
}
