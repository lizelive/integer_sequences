/// a(n) = 4*T(n)^1
/// https://oeis.org/A000803

pub struct A000803;

impl crate::traits::IntegerSequence for A000803 {
    const NAME: &str = "a(n) = 4*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 4, 12, 24, 40, 60, 84, 112, 144, 180, 220, 264, 312, 364, 420, 480, 544, 612, 684, 760, 840, 924, 1012, 1104, 1200, 1300, 1404, 1512, 1624, 1740
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000803";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_803(n)
    }
}

const fn tri_pow_803(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    4 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000803>();
}
