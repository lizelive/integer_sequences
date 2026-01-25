/// a(n) = n^3 + 4*n^2 + 4*n + 0
/// https://oeis.org/A000224

pub struct A000224;

impl crate::traits::IntegerSequence for A000224 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 9, 32, 75, 144, 245, 384, 567, 800, 1089, 1440, 1859, 2352, 2925, 3584, 4335, 5184, 6137, 7200, 8379, 9680, 11109, 12672, 14375, 16224
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000224";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_224(n)
    }
}

const fn poly_224(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000224>();
}
