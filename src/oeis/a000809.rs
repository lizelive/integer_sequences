/// a(n) = 10*T(n)^1
/// https://oeis.org/A000809

pub struct A000809;

impl crate::traits::IntegerSequence for A000809 {
    const NAME: &str = "a(n) = 10*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 10, 30, 60, 100, 150, 210, 280, 360, 450, 550, 660, 780, 910, 1050, 1200, 1360, 1530, 1710, 1900, 2100, 2310, 2530, 2760, 3000, 3250, 3510, 3780, 4060, 4350
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000809";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_809(n)
    }
}

const fn tri_pow_809(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    10 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000809>();
}
