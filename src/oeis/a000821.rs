/// a(n) = 2*T(n)^3
/// https://oeis.org/A000821

pub struct A000821;

impl crate::traits::IntegerSequence for A000821 {
    const NAME: &str = "a(n) = 2*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 2, 54, 432, 2000, 6750, 18522, 43904, 93312, 182250, 332750, 574992, 949104, 1507142, 2315250, 3456000, 5030912, 7163154, 10000422, 13718000, 18522000, 24652782, 32388554, 42049152, 54000000, 68656250, 86487102, 108020304, 133846832, 164625750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000821";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_821(n)
    }
}

const fn tri_pow_821(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    2 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000821>();
}
