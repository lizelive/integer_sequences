/// a(n) = 3*T(n) + 7
/// https://oeis.org/A000377

pub struct A000377;

impl crate::traits::IntegerSequence for A000377 {
    const NAME: &str = "a(n) = 3*T(n) + 7";

    const HEAD: &[crate::Value] = &[
        7, 10, 16, 25, 37, 52, 70, 91, 115, 142, 172, 205, 241, 280, 322, 367, 415, 466, 520, 577, 637, 700, 766, 835, 907
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000377";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_377(n)
    }
}

const fn tri_377(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000377>();
}
