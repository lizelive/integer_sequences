/// a(n) = 2*T(n)^2
/// https://oeis.org/A000811

pub struct A000811;

impl crate::traits::IntegerSequence for A000811 {
    const NAME: &str = "a(n) = 2*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 2, 18, 72, 200, 450, 882, 1568, 2592, 4050, 6050, 8712, 12168, 16562, 22050, 28800, 36992, 46818, 58482, 72200, 88200, 106722, 128018, 152352, 180000, 211250, 246402, 285768, 329672, 378450
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000811";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_811(n)
    }
}

const fn tri_pow_811(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    2 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000811>();
}
