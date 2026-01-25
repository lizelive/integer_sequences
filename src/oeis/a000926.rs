/// a(n) = 7*n^2 + 2
/// https://oeis.org/A000926

pub struct A000926;

impl crate::traits::IntegerSequence for A000926 {
    const NAME: &str = "a(n) = 7*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 30, 65, 114, 177, 254, 345, 450, 569, 702, 849, 1010, 1185, 1374, 1577, 1794, 2025, 2270, 2529, 2802, 3089, 3390, 3705, 4034, 4377, 4734, 5105, 5490, 5889
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000926";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_926(n)
    }
}

const fn sq_926(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000926>();
}
