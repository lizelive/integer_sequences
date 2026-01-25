/// a(n) = 8*n^2 + 2
/// https://oeis.org/A000927

pub struct A000927;

impl crate::traits::IntegerSequence for A000927 {
    const NAME: &str = "a(n) = 8*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 10, 34, 74, 130, 202, 290, 394, 514, 650, 802, 970, 1154, 1354, 1570, 1802, 2050, 2314, 2594, 2890, 3202, 3530, 3874, 4234, 4610, 5002, 5410, 5834, 6274, 6730
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000927";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_927(n)
    }
}

const fn sq_927(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000927>();
}
