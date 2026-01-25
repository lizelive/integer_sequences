/// a(n) = 1*T(n)^4
/// https://oeis.org/A000830

pub struct A000830;

impl crate::traits::IntegerSequence for A000830 {
    const NAME: &str = "a(n) = 1*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 1, 81, 1296, 10000, 50625, 194481, 614656, 1679616, 4100625, 9150625, 18974736, 37015056, 68574961, 121550625, 207360000, 342102016, 547981281, 855036081, 1303210000, 1944810000, 2847396321, 4097152081, 5802782976, 8100000000, 11156640625, 15178486401, 20415837456, 27170906896, 35806100625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000830";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_830(n)
    }
}

const fn tri_pow_830(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    1 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000830>();
}
