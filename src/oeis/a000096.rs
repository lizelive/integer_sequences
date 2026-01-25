/// a(n) = n!/(n/2)!^2 if n is even, else 0.
/// https://oeis.org/A000096

pub struct A000096;

impl crate::traits::IntegerSequence for A000096 {
    const NAME: &str = "a(n) = n*(n+3)/2";

    const HEAD: &[crate::Value] = &[
        0, 2, 5, 9, 14, 20, 27, 35, 44, 54, 65, 77, 90, 104, 119, 135, 152, 170, 189, 209, 230,
        252, 275, 299, 324, 350, 377, 405, 434, 464, 495, 527, 560, 594, 629, 665, 702, 740, 779,
        819, 860, 902, 945, 989, 1034, 1080, 1127, 1175, 1224, 1274, 1325, 1377, 1430, 1484, 1539,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000096";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        n * (n + 3) / 2
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000096>();
}
