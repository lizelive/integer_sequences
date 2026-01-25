/// a(n) = 7*n^2 + 6
/// https://oeis.org/A000966

pub struct A000966;

impl crate::traits::IntegerSequence for A000966 {
    const NAME: &str = "a(n) = 7*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 13, 34, 69, 118, 181, 258, 349, 454, 573, 706, 853, 1014, 1189, 1378, 1581, 1798, 2029, 2274, 2533, 2806, 3093, 3394, 3709, 4038, 4381, 4738, 5109, 5494, 5893
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000966";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_966(n)
    }
}

const fn sq_966(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000966>();
}
