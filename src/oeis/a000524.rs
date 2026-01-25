/// a(n) = n^3 + 4*n + 2
/// https://oeis.org/A000524

pub struct A000524;

impl crate::traits::IntegerSequence for A000524 {
    const NAME: &str = "a(n) = n^3 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 18, 41, 82, 147, 242, 373, 546, 767, 1042, 1377, 1778, 2251, 2802, 3437, 4162, 4983, 5906, 6937, 8082, 9347, 10738, 12261, 13922, 15727, 17682, 19793, 22066, 24507
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000524";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_524(n)
    }
}

const fn poly_524(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000524>();
}
