/// a(n) = 6*T(n)^3
/// https://oeis.org/A000825

pub struct A000825;

impl crate::traits::IntegerSequence for A000825 {
    const NAME: &str = "a(n) = 6*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 6, 162, 1296, 6000, 20250, 55566, 131712, 279936, 546750, 998250, 1724976, 2847312, 4521426, 6945750, 10368000, 15092736, 21489462, 30001266, 41154000, 55566000, 73958346, 97165662, 126147456, 162000000, 205968750, 259461306, 324060912, 401540496, 493877250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000825";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_825(n)
    }
}

const fn tri_pow_825(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    6 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000825>();
}
