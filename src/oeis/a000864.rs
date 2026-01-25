/// a(n) = 5*T(n)^2 + 1
/// https://oeis.org/A000864

pub struct A000864;

impl crate::traits::IntegerSequence for A000864 {
    const NAME: &str = "a(n) = 5*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 46, 181, 501, 1126, 2206, 3921, 6481, 10126, 15126, 21781, 30421, 41406, 55126, 72001, 92481, 117046, 146206, 180501, 220501, 266806, 320046, 380881, 450001, 528126, 616006, 714421, 824181, 946126
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000864";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_864(n)
    }
}

const fn tri_pow_864(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    5 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000864>();
}
