/// a(n) = 2*T(n)^3 + 1
/// https://oeis.org/A000871

pub struct A000871;

impl crate::traits::IntegerSequence for A000871 {
    const NAME: &str = "a(n) = 2*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 55, 433, 2001, 6751, 18523, 43905, 93313, 182251, 332751, 574993, 949105, 1507143, 2315251, 3456001, 5030913, 7163155, 10000423, 13718001, 18522001, 24652783, 32388555, 42049153, 54000001, 68656251, 86487103, 108020305, 133846833, 164625751
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000871";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_871(n)
    }
}

const fn tri_pow_871(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    2 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000871>();
}
