/// Triangular numbers: a(n) = C(n+1,2) = n*(n+1)/2 = 0 + 1 + 2 + ... + n.
/// https://oeis.org/A000217

pub struct A000217;

impl crate::traits::IntegerSequence for A000217 {
    const NAME: &str = "Triangular numbers: a(n) = n*(n+1)/2";

    const HEAD: &[crate::Value] = &[
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231,
        253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780,
        820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378, 1431, 1485, 1540,
        1596, 1653, 1711, 1770, 1830, 1891, 1953, 2016, 2080, 2145, 2211, 2278, 2346, 2415, 2485,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000217";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        n * (n + 1) / 2
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000217>();
}
