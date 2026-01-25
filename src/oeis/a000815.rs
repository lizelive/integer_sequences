/// a(n) = 6*T(n)^2
/// https://oeis.org/A000815

pub struct A000815;

impl crate::traits::IntegerSequence for A000815 {
    const NAME: &str = "a(n) = 6*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 6, 54, 216, 600, 1350, 2646, 4704, 7776, 12150, 18150, 26136, 36504, 49686, 66150, 86400, 110976, 140454, 175446, 216600, 264600, 320166, 384054, 457056, 540000, 633750, 739206, 857304, 989016, 1135350
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000815";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_815(n)
    }
}

const fn tri_pow_815(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    6 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000815>();
}
