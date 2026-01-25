/// a(n) = 5*n^2 + 6
/// https://oeis.org/A000964

pub struct A000964;

impl crate::traits::IntegerSequence for A000964 {
    const NAME: &str = "a(n) = 5*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 11, 26, 51, 86, 131, 186, 251, 326, 411, 506, 611, 726, 851, 986, 1131, 1286, 1451, 1626, 1811, 2006, 2211, 2426, 2651, 2886, 3131, 3386, 3651, 3926, 4211
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000964";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_964(n)
    }
}

const fn sq_964(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000964>();
}
