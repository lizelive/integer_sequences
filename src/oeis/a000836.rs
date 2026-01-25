/// a(n) = 7*T(n)^4
/// https://oeis.org/A000836

pub struct A000836;

impl crate::traits::IntegerSequence for A000836 {
    const NAME: &str = "a(n) = 7*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 7, 567, 9072, 70000, 354375, 1361367, 4302592, 11757312, 28704375, 64054375, 132823152, 259105392, 480024727, 850854375, 1451520000, 2394714112, 3835868967, 5985252567, 9122470000, 13613670000, 19931774247, 28680064567, 40619480832, 56700000000, 78096484375, 106249404807, 142910862192, 190196348272, 250642704375
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000836";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_836(n)
    }
}

const fn tri_pow_836(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    7 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000836>();
}
