/// a(n) = 8*T(n)^5
/// https://oeis.org/A000847

pub struct A000847;

impl crate::traits::IntegerSequence for A000847 {
    const NAME: &str = "a(n) = 8*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 8, 1944, 62208, 800000, 6075000, 32672808, 137682944, 483729408, 1476225000, 4026275000, 10018660608, 23097394944, 49922571608, 102102525000, 199065600000, 372206993408, 670729087944, 1169689358808, 1980879200000, 3267280800000, 5261988401208, 8292635811944, 12812544811008, 19440000000000, 29007265625000, 42621189814008, 61737492466944, 88251105598208, 124605230175000
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000847";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_847(n)
    }
}

const fn tri_pow_847(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    8 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000847>();
}
