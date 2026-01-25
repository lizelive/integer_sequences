/// a(n) = 8*T(n)^2
/// https://oeis.org/A000817

pub struct A000817;

impl crate::traits::IntegerSequence for A000817 {
    const NAME: &str = "a(n) = 8*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 8, 72, 288, 800, 1800, 3528, 6272, 10368, 16200, 24200, 34848, 48672, 66248, 88200, 115200, 147968, 187272, 233928, 288800, 352800, 426888, 512072, 609408, 720000, 845000, 985608, 1143072, 1318688, 1513800
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000817";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_817(n)
    }
}

const fn tri_pow_817(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    8 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000817>();
}
