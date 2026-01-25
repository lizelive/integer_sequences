/// a(n) = 8*n^2 + 3
/// https://oeis.org/A000937

pub struct A000937;

impl crate::traits::IntegerSequence for A000937 {
    const NAME: &str = "a(n) = 8*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 11, 35, 75, 131, 203, 291, 395, 515, 651, 803, 971, 1155, 1355, 1571, 1803, 2051, 2315, 2595, 2891, 3203, 3531, 3875, 4235, 4611, 5003, 5411, 5835, 6275, 6731
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000937";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_937(n)
    }
}

const fn sq_937(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000937>();
}
