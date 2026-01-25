/// a(n) = (n+2)*(n+1)^2*n/12
/// https://oeis.org/A000116

pub struct A000116;

impl crate::traits::IntegerSequence for A000116 {
    const NAME: &str = "a(n) = (n+2)*(n+1)^2*n/12";

    const HEAD: &[crate::Value] = &[
        0, 1, 6, 20, 50, 105, 196, 336, 540, 825, 1210, 1716, 2366, 3185, 4200, 5440, 6936, 8721
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000116";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        formula_116(n)
    }
}

const fn formula_116(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    (n + 2) * (n + 1) * (n + 1) * n / 12
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000116>();
}
