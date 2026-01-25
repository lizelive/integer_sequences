/// a(n) = 4*T(n) + 3
/// https://oeis.org/A000383

pub struct A000383;

impl crate::traits::IntegerSequence for A000383 {
    const NAME: &str = "a(n) = 4*T(n) + 3";

    const HEAD: &[crate::Value] = &[
        3, 7, 15, 27, 43, 63, 87, 115, 147, 183, 223, 267, 315, 367, 423, 483, 547, 615, 687, 763, 843, 927, 1015, 1107, 1203
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000383";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_383(n)
    }
}

const fn tri_383(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000383>();
}
