/// a(n) = 1*T(n)^4 + 1
/// https://oeis.org/A000880

pub struct A000880;

impl crate::traits::IntegerSequence for A000880 {
    const NAME: &str = "a(n) = 1*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 82, 1297, 10001, 50626, 194482, 614657, 1679617, 4100626, 9150626, 18974737, 37015057, 68574962, 121550626, 207360001, 342102017, 547981282, 855036082, 1303210001, 1944810001, 2847396322, 4097152082, 5802782977, 8100000001, 11156640626, 15178486402, 20415837457, 27170906897, 35806100626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000880";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_880(n)
    }
}

const fn tri_pow_880(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    1 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000880>();
}
