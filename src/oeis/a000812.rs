/// a(n) = 3*T(n)^2
/// https://oeis.org/A000812

pub struct A000812;

impl crate::traits::IntegerSequence for A000812 {
    const NAME: &str = "a(n) = 3*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 3, 27, 108, 300, 675, 1323, 2352, 3888, 6075, 9075, 13068, 18252, 24843, 33075, 43200, 55488, 70227, 87723, 108300, 132300, 160083, 192027, 228528, 270000, 316875, 369603, 428652, 494508, 567675
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000812";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_812(n)
    }
}

const fn tri_pow_812(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    3 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000812>();
}
