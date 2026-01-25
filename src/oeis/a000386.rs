/// a(n) = 4*T(n) + 6
/// https://oeis.org/A000386

pub struct A000386;

impl crate::traits::IntegerSequence for A000386 {
    const NAME: &str = "a(n) = 4*T(n) + 6";

    const HEAD: &[crate::Value] = &[
        6, 10, 18, 30, 46, 66, 90, 118, 150, 186, 226, 270, 318, 370, 426, 486, 550, 618, 690, 766, 846, 930, 1018, 1110, 1206
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000386";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_386(n)
    }
}

const fn tri_386(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000386>();
}
