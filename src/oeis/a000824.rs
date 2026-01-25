/// a(n) = 5*T(n)^3
/// https://oeis.org/A000824

pub struct A000824;

impl crate::traits::IntegerSequence for A000824 {
    const NAME: &str = "a(n) = 5*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 5, 135, 1080, 5000, 16875, 46305, 109760, 233280, 455625, 831875, 1437480, 2372760, 3767855, 5788125, 8640000, 12577280, 17907885, 25001055, 34295000, 46305000, 61631955, 80971385, 105122880, 135000000, 171640625, 216217755, 270050760, 334617080, 411564375
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000824";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_824(n)
    }
}

const fn tri_pow_824(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    5 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000824>();
}
