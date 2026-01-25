/// a(n) = 3*T(n) + 1
/// https://oeis.org/A000371

pub struct A000371;

impl crate::traits::IntegerSequence for A000371 {
    const NAME: &str = "a(n) = 3*T(n) + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 10, 19, 31, 46, 64, 85, 109, 136, 166, 199, 235, 274, 316, 361, 409, 460, 514, 571, 631, 694, 760, 829, 901
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000371";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_371(n)
    }
}

const fn tri_371(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000371>();
}
