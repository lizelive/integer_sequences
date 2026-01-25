/// a(n) = 7*n^2
/// https://oeis.org/A000256

pub struct A000256;

impl crate::traits::IntegerSequence for A000256 {
    const NAME: &str = "a(n) = 7*n^2";

    const HEAD: &[crate::Value] = &[
        0, 7, 28, 63, 112, 175, 252, 343, 448, 567, 700, 847, 1008, 1183, 1372, 1575, 1792, 2023, 2268, 2527, 2800, 3087, 3388, 3703, 4032
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000256";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_256(n)
    }
}

const fn power_256(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000256>();
}
