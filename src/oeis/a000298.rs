/// a(n) = 9*n^6
/// https://oeis.org/A000298

pub struct A000298;

impl crate::traits::IntegerSequence for A000298 {
    const NAME: &str = "a(n) = 9*n^6";

    const HEAD: &[crate::Value] = &[
        0, 9, 576, 6561, 36864, 140625, 419904, 1058841, 2359296, 4782969, 9000000, 15944049, 26873856, 43441281, 67765824, 102515625, 150994944, 217238121, 306110016, 423412929, 576000000, 771895089, 1020419136, 1332323001, 1719926784
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000298";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_298(n)
    }
}

const fn power_298(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000298>();
}
