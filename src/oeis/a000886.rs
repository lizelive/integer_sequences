/// a(n) = 7*T(n)^4 + 1
/// https://oeis.org/A000886

pub struct A000886;

impl crate::traits::IntegerSequence for A000886 {
    const NAME: &str = "a(n) = 7*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 568, 9073, 70001, 354376, 1361368, 4302593, 11757313, 28704376, 64054376, 132823153, 259105393, 480024728, 850854376, 1451520001, 2394714113, 3835868968, 5985252568, 9122470001, 13613670001, 19931774248, 28680064568, 40619480833, 56700000001, 78096484376, 106249404808, 142910862193, 190196348273, 250642704376
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000886";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_886(n)
    }
}

const fn tri_pow_886(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    7 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000886>();
}
