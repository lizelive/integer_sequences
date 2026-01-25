/// a(n) = 1*T(n) + 2
/// https://oeis.org/A000352

pub struct A000352;

impl crate::traits::IntegerSequence for A000352 {
    const NAME: &str = "a(n) = 1*T(n) + 2";

    const HEAD: &[crate::Value] = &[
        2, 3, 5, 8, 12, 17, 23, 30, 38, 47, 57, 68, 80, 93, 107, 122, 138, 155, 173, 192, 212, 233, 255, 278, 302
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000352";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_352(n)
    }
}

const fn tri_352(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000352>();
}
