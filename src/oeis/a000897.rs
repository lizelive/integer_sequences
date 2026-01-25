/// a(n) = 8*T(n)^5 + 1
/// https://oeis.org/A000897

pub struct A000897;

impl crate::traits::IntegerSequence for A000897 {
    const NAME: &str = "a(n) = 8*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 1945, 62209, 800001, 6075001, 32672809, 137682945, 483729409, 1476225001, 4026275001, 10018660609, 23097394945, 49922571609, 102102525001, 199065600001, 372206993409, 670729087945, 1169689358809, 1980879200001, 3267280800001, 5261988401209, 8292635811945, 12812544811009, 19440000000001, 29007265625001, 42621189814009, 61737492466945, 88251105598209, 124605230175001
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000897";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_897(n)
    }
}

const fn tri_pow_897(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    8 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000897>();
}
