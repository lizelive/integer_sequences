/// a(n) = 2*n^2 + 2*n + 3
/// https://oeis.org/A000741

pub struct A000741;

impl crate::traits::IntegerSequence for A000741 {
    const NAME: &str = "a(n) = 2*n^2 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 7, 15, 27, 43, 63, 87, 115, 147, 183, 223, 267, 315, 367, 423, 483, 547, 615, 687, 763, 843, 927, 1015, 1107, 1203, 1303, 1407, 1515, 1627, 1743
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000741";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_741(n)
    }
}

const fn quad_741(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000741>();
}
