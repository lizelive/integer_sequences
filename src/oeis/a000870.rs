/// a(n) = 1*T(n)^3 + 1
/// https://oeis.org/A000870

pub struct A000870;

impl crate::traits::IntegerSequence for A000870 {
    const NAME: &str = "a(n) = 1*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 28, 217, 1001, 3376, 9262, 21953, 46657, 91126, 166376, 287497, 474553, 753572, 1157626, 1728001, 2515457, 3581578, 5000212, 6859001, 9261001, 12326392, 16194278, 21024577, 27000001, 34328126, 43243552, 54010153, 66923417, 82312876
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000870";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_870(n)
    }
}

const fn tri_pow_870(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    1 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000870>();
}
